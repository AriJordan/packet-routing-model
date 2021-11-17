from numpy import array
from math import floor, ceil
from argparse import ArgumentParser

from numpy.core.function_base import linspace
from multiFlowClass import MultiFlow
from datetime import datetime
import numpy as np
import matplotlib.pyplot as plt
import pickle
import sys
import os
import timeit
import json
import subprocess

def load_graph(path):
    """
    Load graph instance from '.cg' file
    :param path: Path to load graph from
    :return: returns networkx graph instance
    """
    with open(path, 'rb') as f:
        network = pickle.load(f)
    return network

def read_files():
    """
    Reads the files and initiates MultiFlow instance
    :param networkFile: networkx graph
    :param inflowFile: File containing commodities
    :return: MultiFlow object
    """
    network = load_graph(network_in_file)
    mf = MultiFlow(network)
    with open(inflow_in_file, 'r') as fRead:
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

def write_discrete_jsons(mf : MultiFlow, alpha : float, beta : float):
    data = {}
    data['edges'] = []
    for (v_from, v_to, ddict) in mf.network.edges(data=True):
        transit_time = float(ddict['transitTime'])
        capacity = float(ddict['outCapacity'])
        EPS = 1e-6
        FRAC_PRECISION = 100800
        # Heuristic way of converting float to fractional
        real_capacity = capacity * alpha / beta 
        numerator = round(FRAC_PRECISION * real_capacity)
        denominator = FRAC_PRECISION
        num_den_gcd = np.gcd(numerator, denominator)
        assert(numerator % num_den_gcd == 0)
        assert(denominator % num_den_gcd == 0)
        data['edges'].append({
            'v_from' : v_from,
            'v_to' : v_to,
            'transit_time' : ceil(transit_time / alpha - EPS),
            'capacity' : {
                'numerator' : int(numerator / num_den_gcd),
                'denominator' : int(denominator / num_den_gcd),
            }
        })
    with open(instance_directory + 'network.json', 'w') as network_out_file:
        json.dump(data, network_out_file, indent=4)
        
    data = {}
    data['packets'] = []
    for (commodity_id, path) in enumerate(mf.pathCommodityDict):
        (start_time, end_time, rate) = mf.pathCommodityDict[path]
        packets_generated = 0
        for current_time in range(ceil(start_time / alpha), floor(end_time / alpha) + 1):
            for packet in range(0, floor((current_time - (start_time / alpha)) * alpha / beta * rate) - packets_generated):
                data['packets'].append({
                    'commodity_id' : commodity_id,
                    'release_time' : current_time,
                    'path' : path,
                })
                packets_generated += 1
    with open(instance_directory + 'packets.json', 'w') as packets_out_file:
        json.dump(data, packets_out_file, indent=4)

def run_packet_routing(mf : MultiFlow, alpha, beta):
    write_discrete_jsons(mf, alpha, beta)
    subprocess.run([rust_executable, instance_directory])
    
def run_multi_flow(mf : MultiFlow):
    mf.compute()
    mf.generate_output(instance_directory, 'multi_flow')

def compare_models(mf : MultiFlow, alpha : float, beta : float):
    with open(instance_directory + 'results.json') as results_file:
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

    #colmaps = plt.get_cmap('autumn', N=len(commodity_ids))
    packet_colors = plt.get_cmap('autumn')(np.linspace(0, 0.8, 2))
    flow_colors = plt.get_cmap('winter')(np.linspace(0, 1, 2))
    for commodity_id in commodity_ids:
        packet_x = [packet_release_times[i] for i in range(len(packet_travel_times)) if packet_commodity_ids[i] == commodity_id]
        packet_y = [packet_travel_times[i] for i in range(len(packet_travel_times)) if packet_commodity_ids[i] == commodity_id]
        plt.plot(packet_x, packet_y, color=packet_colors[commodity_id], marker='s', linestyle='none')
        flow_x = [flow_release_times[i] for i in range(len(flow_travel_times)) if flow_commodity_ids[i] == commodity_id]
        flow_y = [flow_travel_times[i] for i in range(len(flow_travel_times)) if flow_commodity_ids[i] == commodity_id]
        plt.plot(flow_x, flow_y, color=flow_colors[commodity_id])
    plt.xlabel('release time')
    plt.ylabel('travel time')
    packet_flow_labels = []
    for commodity_id in commodity_ids:
        packet_flow_labels.append('packets ' + str(commodity_id))
        packet_flow_labels.append('flow ' + str(commodity_id))
    plt.legend(packet_flow_labels)
    #plt.savefig(datetime.now().strftime("plots\\a" + str(alpha) + "_b" + str(beta) + "comparison_%d-%m-%Y_%H;%M;%S"))
    plt.show()

def multiple_runs(mf, alphas, betas):
    for alpha in alphas:
        for beta in betas:
            run_packet_routing(mf, alpha, beta)
            compare_models(mf, alpha, beta)

# Parse instance name
parser = ArgumentParser(description='Compare Packet Routing to Multi-Commodity Flows Over Time')
parser.add_argument('instance_name', type=str, help='name of instance to run')
args = parser.parse_args()
INSTANCE_NAME = args.instance_name

# Files containing network and flow data
instance_directory : str = 'src/instances/' + INSTANCE_NAME + '/'
network_in_file : str = instance_directory + 'network.cg'
inflow_in_file : str = instance_directory + 'inflow.txt'
rust_executable = 'target/debug/routing.exe'

mf : MultiFlow = read_files()

ALPHAS = [1, 0.5]
BETAS = [1, 0.5]
run_multi_flow(mf)
multiple_runs(mf, ALPHAS, BETAS)
