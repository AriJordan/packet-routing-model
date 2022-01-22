import numpy as np
import matplotlib.pyplot as plt
import pickle
import sys
import json
import subprocess
from numpy import array, interp, zeros
from mpl_toolkits import mplot3d
from math import floor, ceil
from datetime import datetime
from .multiFlowClass import MultiFlow


class Results():
    def __init__(self, packet_release_times, packet_travel_times, packet_commodity_ids,
            flow_release_times, flow_travel_times, flow_commodity_ids):
        self.packet_release_times = packet_release_times
        self.packet_travel_times = packet_travel_times
        self.packet_commodity_ids = packet_commodity_ids
        self.flow_release_times = flow_release_times
        self.flow_travel_times = flow_travel_times
        self.flow_commodity_ids = flow_commodity_ids
        
class Simulation():
    def __init__(self, INSTANCE_NAME):
        self.instance_name = INSTANCE_NAME
        # Directory containing network and flow data
        self.instance_directory : str = "src/instances/" + INSTANCE_NAME + "/"
        # Directory containing simulation executable
        self.rust_executable_directory = "target/release/"
        if self.rust_executable_directory == "target/debug/":
            print("Running in DEBUG mode")
        else:
            assert self.rust_executable_directory == "target/release/"
            print("Running in RELEASE mode")
        
    def load_graph(self, path):
        """
        Load graph instance from ".cg" file
        :param path: Path to load graph from
        :return: returns networkx graph instance
        """
        with open(path, "rb") as f:
            network = pickle.load(f)
        return network

    def read_files(self):
        """
        Reads the files and initiates MultiFlow instance
        :param networkFile: networkx graph
        :param inflowFile: File containing commodities
        :return: MultiFlow object
        """
        network = self.load_graph(self.instance_directory + "network.cg")
        mf = MultiFlow(network)
        with open(self.instance_directory + "inflow.txt", 'r') as fRead:
            firstLine = True
            for line in fRead:
                if firstLine:
                    firstLine = False
                else:
                    line = line.strip()
                    rate, interval, path = line.split()
                    startTime, endTime = interval.split(",")
                    path = tuple(path.split(","))
                    mf.add_commodity(path, float(startTime), float(endTime), float(rate))

        rc = mf.validate_input()
        if rc != 0:
            # Return code is error message
            sys.exit(rc)

        return mf

    def write_discrete_jsons(self, mf : MultiFlow, alpha : float, beta : float):
        data = {}
        data["edges"] = []
        for (v_from, v_to, ddict) in mf.network.edges(data=True):
            transit_time = float(ddict["transitTime"])
            capacity = float(ddict["outCapacity"])
            EPS = 1e-6
            FRAC_PRECISION = 100800
            # Heuristic way of converting float to fractional
            real_capacity = capacity * alpha / beta
            numerator = round(FRAC_PRECISION * real_capacity)
            denominator = FRAC_PRECISION
            num_den_gcd = np.gcd(numerator, denominator)
            assert(numerator % num_den_gcd == 0)
            assert(denominator % num_den_gcd == 0)
            data["edges"].append({
                "v_from" : v_from,
                "v_to" : v_to,
                "transit_time" : ceil(transit_time / alpha - EPS),
                "capacity" : {
                    "numerator" : int(numerator / num_den_gcd),
                    "denominator" : int(denominator / num_den_gcd),
                }
            })
        with open(self.instance_directory + "network.json", 'w') as network_out_file:
            json.dump(data, network_out_file, indent=4)
            
        data = {}
        data["packets"] = []
        for (commodity_id, path) in enumerate(mf.pathCommodityDict):
            (start_time, end_time, rate) = mf.pathCommodityDict[path]
            packets_generated = 0
            for current_time in range(ceil(start_time / alpha), floor(end_time / alpha) + 1):
                for packet in range(0, floor((current_time - (start_time / alpha)) * alpha / beta * rate + EPS) - packets_generated):
                    data["packets"].append({
                        "commodity_id" : commodity_id,
                        "release_time" : current_time,
                        "path" : path,
                    })
                    packets_generated += 1
        with open(self.instance_directory + "packets.json", 'w') as packets_out_file:
            json.dump(data, packets_out_file, indent=4)

    def run_packet_routing(self, mf : MultiFlow, alpha, beta):
        print(f'running packet routing simulation with alpha={alpha}, beta={beta}')
        self.write_discrete_jsons(mf, alpha, beta)
        try: 
            subprocess.run([self.rust_executable_directory + "routing.exe", self.instance_directory])
        except:
            print(f"running routing executable under directory {self.rust_executable_directory} failed")
        
    def run_multi_flow(self):
        mf : MultiFlow = self.read_files()
        mf.compute()
        mf.generate_output(self.instance_directory, "multi_flow")
        return mf

    def compare_models(self, mf : MultiFlow, alpha : float, beta : float):
        with open(self.instance_directory + "results.json") as results_file:
            packet_results = json.load(results_file)
            packet_commodity_ids = array(packet_results["commodity_ids"])
            commodity_ids = list(set(packet_commodity_ids))
            packet_arrival_times = array(packet_results["arrival_times"]) * alpha
            packet_travel_times = array(packet_results["travel_times"]) * alpha
            packet_release_times = packet_arrival_times - packet_travel_times
            n_packets = len(packet_arrival_times)
        flow_travel_times = []
        flow_release_times = []
        flow_commodity_ids = []
        for (commodity_id, path) in enumerate(mf.pathCommodityDict):
            start_time, end_time, rate = mf.pathCommodityDict[path]
            time_points = sorted(list(set([start_time, end_time, *mf.get_break_points(path)])))
            flow_travel_times.extend([mf.path_travel_time(path, t) for t in time_points])
            flow_release_times.extend(t for t in time_points)
            flow_commodity_ids.extend([commodity_id for _ in time_points])

        results = Results(
            packet_release_times, packet_travel_times, packet_commodity_ids,
            flow_release_times, flow_travel_times, flow_commodity_ids)
        return results

    def plot_packets_vs_flow(self, results: Results, alpha, beta, show_plot : bool, save_plot : bool):
        #colmaps = plt.get_cmap("autumn", N=len(commodity_ids))
        commodity_ids = list(set(results.packet_commodity_ids))
        packet_colors = plt.get_cmap("autumn")(np.linspace(0, 0.8, len(commodity_ids)))
        flow_colors = plt.get_cmap("winter")(np.linspace(0, 1, len(commodity_ids)))
        max_release_time = 0
        for commodity_id in commodity_ids:
            packet_x = [results.packet_release_times[i] for i in range(len(results.packet_travel_times)) if results.packet_commodity_ids[i] == commodity_id]
            max_release_time = max(max_release_time, max(packet_x))
            packet_y = [results.packet_travel_times[i] for i in range(len(results.packet_travel_times)) if results.packet_commodity_ids[i] == commodity_id]
            plt.plot(packet_x, packet_y, color=packet_colors[commodity_id], marker='s', linestyle="none")
            flow_x = [results.flow_release_times[i] for i in range(len(results.flow_travel_times)) if results.flow_commodity_ids[i] == commodity_id]
            flow_y = [results.flow_travel_times[i] for i in range(len(results.flow_travel_times)) if results.flow_commodity_ids[i] == commodity_id]
            plt.plot(flow_x, flow_y, color=flow_colors[commodity_id])
        plt.title(f"packets vs flow travel times, a={alpha}, b={beta}")
        plt.xlabel("release time")
        plt.xlim(right=max_release_time * 1.01)
        plt.ylabel("travel time")
        packet_flow_labels = []
        for commodity_id in commodity_ids:
            packet_flow_labels.append("packets " + str(commodity_id + 1))
            packet_flow_labels.append("flow " + str(commodity_id + 1))
        plt.legend(packet_flow_labels)
        if save_plot:
            salpha = str(alpha).replace(".", "-")
            sbeta = str(beta).replace(".", "-")
            plt.savefig(datetime.now().strftime(f"plots\\{self.instance_name}_a{salpha}_b{sbeta}_packets_vs_flow_%d-%m-%Y_%H-%M-%S"))
        if show_plot:
            plt.show()

    def error_norm(self, errors):
        return np.max(np.abs(errors)), "maximum error"

    def calc_approx_error(self, results : Results):
        n_packets = len(results.packet_release_times)
        errors = []
        for commodity_id in sorted(list(set(results.packet_commodity_ids))):
            commodity_packet_ids = [i for i in range(n_packets) if results.packet_commodity_ids[i] == commodity_id]
            commodity_flow_ids = [i for i in range(len(results.flow_commodity_ids)) if results.flow_commodity_ids[i] == commodity_id]
            commodity_flow_release_times = [results.flow_release_times[i] for i in commodity_flow_ids]
            commodity_flow_travel_times = [results.flow_travel_times[i] for i in commodity_flow_ids]
            for packet_id in commodity_packet_ids:
                errors.append(results.packet_travel_times[packet_id] - 
                    interp(x=results.packet_release_times[packet_id], xp=commodity_flow_release_times, fp=commodity_flow_travel_times))
        return self.error_norm(errors)

    def plot_approx_errors_1D(self, approx_errors, alphas, show_plot : bool, save_plot : bool, description : str, error_description):
        plt.title(description)
        plt.xlabel("alpha")
        plt.ylabel(error_description)
        plt.plot(alphas, approx_errors)
        if save_plot:
            plt.savefig(datetime.now().strftime(f"plots\\{self.instance_name}_approx_errors_1D_%d-%m-%Y_%H-%M-%S"))
        if show_plot:
            plt.show()
    
    def plot_approx_errors_2D(self, approx_errors, alphas, betas, show_plot, save_plot, description, error_description):
        ax = plt.axes(projection='3d')
        ax.set_title(description)
        ax.set_xlabel("beta")
        ax.set_ylabel("alpha")
        ax.set_zlabel(error_description)
        X, Y = np.meshgrid(betas, alphas)
        ax.plot_surface(X, Y, approx_errors, rstride=1, cstride=1,
                cmap='viridis', edgecolor='none')
        if save_plot:
            plt.savefig(datetime.now().strftime(f"plots\\{self.instance_name}_approx_errors_2D_%d-%m-%Y_%H-%M-%S"))
        if show_plot:
            plt.show()

def single_run(INSTANCE_NAME : str, alpha, beta, show_plot : bool, save_plot : bool):
    simulation = Simulation(INSTANCE_NAME)
    mf = simulation.run_multi_flow()
    simulation.run_packet_routing(mf, alpha, beta)
    results = simulation.compare_models(mf, alpha, beta)
    simulation.plot_packets_vs_flow(results, alpha, beta, show_plot, save_plot)

def multiple_runs(INSTANCE_NAME : str, alphas, betas, show_plot : bool, save_plot : bool, description : str):
    assert len(alphas) == len(betas), "List of alphas should have same length as list of betas"
    simulation = Simulation(INSTANCE_NAME)
    mf = simulation.run_multi_flow()
    n_runs = len(alphas)
    approx_errors = zeros(n_runs)
    for run_id in range(n_runs):
        simulation.run_packet_routing(mf, alphas[run_id], betas[run_id])
        results = simulation.compare_models(mf, alphas[run_id], betas[run_id])
        approx_errors[run_id], error_description = simulation.calc_approx_error(results)
    simulation.plot_approx_errors_1D(approx_errors, alphas, show_plot, save_plot, description, error_description)

def multiple_runs2D(INSTANCE_NAME : str, alphas, betas, show_plot : bool, save_plot : bool, description : str):
    simulation = Simulation(INSTANCE_NAME)
    mf = simulation.run_multi_flow()
    n_alphas, n_betas = len(alphas), len(betas)
    approx_errors = zeros((n_alphas, n_betas))
    for alpha_id in range(n_alphas):
        for beta_id in range(n_betas):
            simulation.run_packet_routing(mf, alphas[alpha_id], betas[beta_id])
            results = simulation.compare_models(mf, alphas[alpha_id], betas[beta_id])
            approx_errors[alpha_id][beta_id], error_description = simulation.calc_approx_error(results)
    simulation.plot_approx_errors_2D(approx_errors, alphas, betas, show_plot, save_plot, description, error_description)
