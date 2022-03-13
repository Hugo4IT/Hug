"""
File: bootstrap.py
Version: 0.1.0
Author: Hugo4IT
License: MIT
Repository: https://github.com/Hugo4IT/Hug

Description: Entry point for bootstrapper located in bootstrap/compiler.py
"""

import sys
import logging
import colorama
import bootstrap.compiler as compiler

LOGGING_FMT_BASE = colorama.Style.DIM + "%(asctime)s %(filename)s:%(lineno)-3d" + colorama.Style.RESET_ALL + " $$LOG_COLOR$$[%(levelname)s]: " + colorama.Style.RESET_ALL + colorama.Fore.LIGHTCYAN_EX + "%(message)s" + colorama.Style.RESET_ALL

class Options:
    def __init__(self):
        self.loglevel = "WARNING"
        self.inputfile = ""
        self.highlight = False

    def finish(self):
        logger = logging.getLogger()
        logger.setLevel(getattr(logging, self.loglevel))

        debughandler = logging.StreamHandler()
        debughandler.setLevel(logging.DEBUG)
        debughandler.addFilter(lambda r: r.levelno == logging.DEBUG)
        debughandler.setFormatter(logging.Formatter(LOGGING_FMT_BASE.replace("$$LOG_COLOR$$", colorama.Style.DIM)))
        logger.addHandler(debughandler)

        infohandler = logging.StreamHandler()
        infohandler.setLevel(logging.INFO)
        infohandler.addFilter(lambda r: r.levelno == logging.INFO)
        infohandler.setFormatter(logging.Formatter(LOGGING_FMT_BASE.replace("$$LOG_COLOR$$", colorama.Style.BRIGHT + colorama.Fore.LIGHTBLUE_EX)))
        logger.addHandler(infohandler)

        warninghandler = logging.StreamHandler()
        warninghandler.setLevel(logging.WARNING)
        warninghandler.addFilter(lambda r: r.levelno == logging.WARNING)
        warninghandler.setFormatter(logging.Formatter(LOGGING_FMT_BASE.replace("$$LOG_COLOR$$", colorama.Style.BRIGHT + colorama.Fore.LIGHTYELLOW_EX)))
        logger.addHandler(warninghandler)

        errorhandler = logging.StreamHandler()
        errorhandler.setLevel(logging.ERROR)
        errorhandler.addFilter(lambda r: r.levelno == logging.ERROR or r.levelno == logging.CRITICAL)
        errorhandler.setFormatter(logging.Formatter(LOGGING_FMT_BASE.replace("$$LOG_COLOR$$", colorama.Style.BRIGHT + colorama.Fore.LIGHTRED_EX)))
        logger.addHandler(errorhandler)

        if self.inputfile == "":
            logging.critical("No input file given!")

def printusage():
    print(colorama.Style.BRIGHT + "Usage:")
    print(colorama.Fore.MAGENTA + "  python3" + colorama.Style.RESET_ALL + colorama.Style.DIM + " ./bootstrap.py" + colorama.Style.RESET_ALL + "[options] <file>")
    print()
    print("Available options:")
    print("  --verbose,-v                Enable verbose output (same as --log-level DEBUG)")
    print("  --help,-h                   Print this help message")
    print("  --version,-V                Print the current version of bootstrap.py")
    print("  --log-level,-l <level>      Increase/decrease output verbosity (<level>: [error, warning, info, debug])")
    print("  --highlight-syntax,-H       Print a syntax highlighted version of your code")

def main():
    options = Options()

    # Parse command line arguments
    currentarg = ""
    for i in range(len(sys.argv) - 1):
        arg = sys.argv[i + 1]
        if currentarg == "":
            if arg.startswith("--") or arg.startswith("-"):
                if arg == "--verbose" or arg == "-v":
                    options.loglevel = "DEBUG"
                elif arg == "--help" or arg == "-h":
                    printusage()
                    return
                elif arg == "--version" or arg == "-V":
                    print("bootstrap.py - v0.1.0")
                    return
                elif arg == "--highlight-syntax" or arg == "-H":
                    options.highlight = True
                else:
                    currentarg = arg
            else:
                options.inputfile = arg
        else:
            if currentarg == "--log-level" or currentarg == "-l":
                options.loglevel = arg.upper()
                currentarg = ""
            else:
                print("Unrecognized argument:", currentarg)
                printusage()
                quit()

    options.finish()

    compiler.compile(options)

if __name__ == "__main__":
    main()
