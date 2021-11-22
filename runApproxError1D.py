from python_impl import callSimulation
from argparse import ArgumentParser

if __name__ == "__main__":
    SHOW_PLOT = True
    SAVE_PLOT = False
    DESCRIPTION = "Error with b = a**2"
    parser = ArgumentParser(description="Compare Packet Routing to Multi-Commodity Flows Over Time")
    parser.add_argument("instance_name", type=str, help="name of instance to run")
    args = parser.parse_args()
    INSTANCE_NAME = args.instance_name

    N = 3
    ALPHAS = [0.5**i for i in range(N)]
    # b = a**2
    BETAS = [ALPHAS[i]**2 for i in range(N)]
    callSimulation.multiple_runs(INSTANCE_NAME=INSTANCE_NAME, alphas=ALPHAS, betas=BETAS,
        show_plot=SHOW_PLOT, save_plot=SAVE_PLOT, description=DESCRIPTION)

