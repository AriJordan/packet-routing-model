from python_impl import callSimulation, parseArgs

if __name__ == "__main__":
    INSTANCE_NAME = parseArgs.parse_instance_name()
    SHOW_PLOT = True
    SAVE_PLOT = True
    N = 100
    DESCRIPTION = "Error as function of alpha and beta"
    ALPHAS = [i / N for i in range(1, N + 1)]
    BETAS = [ALPHAS[i] for i in range(N)]
    print(BETAS)
    callSimulation.multiple_runs2D(INSTANCE_NAME=INSTANCE_NAME, alphas=ALPHAS, betas=BETAS,
        show_plot=SHOW_PLOT, save_plot=SAVE_PLOT, description=DESCRIPTION)
