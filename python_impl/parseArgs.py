from argparse import ArgumentParser

def parse_instance_name():
    # Parse only instance name
    parser = ArgumentParser(description="Compare Packet Routing to Multi-Commodity Flows Over Time")
    parser.add_argument("instance_name", type=str, help="name of instance to run")
    args = parser.parse_args()
    return args.instance_name

def parse_instance_name_alpha_beta():
    # Parse instance name and optionally alpha and/or beta
    parser = ArgumentParser(description="Compare Packet Routing to Multi-Commodity Flows Over Time")
    parser.add_argument("instance_name", type=str, help="name of instance to run")
    parser.add_argument("--alpha", default=1, type=float, help="time step size")
    parser.add_argument("--beta", default=1, type=float, help="packet size")
    args = parser.parse_args()
    return args.instance_name, args.alpha, args.beta
