from multiFlowClass import MultiFlow
from math import floor, ceil
import pickle
import sys
import os
import timeit
import json
import subprocess

folder_path : str = 'src/instances/instance_zimmer/'
network_in_file : str = folder_path + 'simple_merge.cg'
inflow_in_file : str = folder_path + 'inflow.txt'
EPS = 1e-9

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
        data['edges'].append({
            'v_from' : v_from,
            'v_to' : v_to,
            'transit_time' : transit_time / alpha,
            'capacity' : capacity,
        })
    with open(folder_path + 'network.json', 'w') as network_out_file:
        json.dump(data, network_out_file, indent=4)
        
    data = {}
    data['packets'] = []
    for path in mf.pathCommodityDict:
        (start_time, end_time, rate) = mf.pathCommodityDict[path]
        packets_generated = 0
        for time in range(ceil(start_time / alpha), floor(end_time / alpha) + 1, alpha):
            for packet in range(0, floor(time * beta / alpha * rate) - packets_generated):
                data['packets'].append({
                    'release_time' : time,
                    'path' : path,
                })
                packets_generated += 1
    with open(folder_path + 'packets.json', 'w') as packets_out_file:
        json.dump(data, packets_out_file, indent=4)

def run_packet_routing(mf : MultiFlow):
    ALPHA = 1
    BETA = 1
    write_discrete_jsons(mf, ALPHA, BETA)
    subprocess.run("target/debug/routing.exe")
    
def run_multi_flow(mf : MultiFlow):
    mf.compute()
    mf.generate_output(folder_path, "multi_flow")
    mf.commodityOutflow
    

mf : MultiFlow = read_files()
run_packet_routing(mf)
run_multi_flow(mf)
