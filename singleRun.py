import os
from python_impl import callSimulation
from argparse import ArgumentParser
if __name__ == "__main__":
    # Parse instance name
    parser = ArgumentParser(description="Compare Packet Routing to Multi-Commodity Flows Over Time")
    parser.add_argument("instance_name", type=str, help="name of instance to run")
    parser.add_argument("--alpha", default=1)
    parser.add_argument("--beta", default=1)
    args = parser.parse_args()
    INSTANCE_NAME = args.instance_name
    alpha = args.alpha
    beta = args.beta
    callSimulation.single_run(INSTANCE_NAME=INSTANCE_NAME, alpha=alpha, beta=beta)