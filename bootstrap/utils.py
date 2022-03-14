import os
import logging
import colorama

def requestfile(filename) -> str:
    if not os.path.exists(filename):
        logging.error("Requested file not found: %s", colorama.Style.BRIGHT + colorama.Fore.LIGHTYELLOW_EX + filename)
        quit()
    else:
        logging.info("File requested: %s", colorama.Style.BRIGHT + colorama.Fore.LIGHTYELLOW_EX + filename)

    contents = ""
    with open(filename, "r") as file:
        contents = file.read()
    
    return contents