from python_impl import callSimulation, parseArgs

if __name__ == "__main__":
    INSTANCE_NAME = parseArgs.parse_instance_name()
    SHOW_PLOT = True
    SAVE_PLOT = False  
    N = 6
    DESCRIPTION = "Error with beta = (alpha)^2"
    ALPHAS = [0.5**i for i in range(N)]
    BETAS = [ALPHAS[i]**2 for i in range(N)] # beta = alpha^2
    callSimulation.multiple_runs(INSTANCE_NAME=INSTANCE_NAME, alphas=ALPHAS, betas=BETAS,
        show_plot=SHOW_PLOT, save_plot=SAVE_PLOT, description=DESCRIPTION)
