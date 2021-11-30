from python_impl import callSimulation, parseArgs

if __name__ == "__main__":
    SHOW_PLOT = True
    SAVE_PLOT = True
    INSTANCE_NAME, ALPHA, BETA = parseArgs.parse_instance_name_alpha_beta()
    callSimulation.single_run(INSTANCE_NAME=INSTANCE_NAME, alpha=ALPHA, beta=BETA, show_plot=SHOW_PLOT, save_plot=SAVE_PLOT)