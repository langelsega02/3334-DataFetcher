# 3334-DataFetcher

This is a project from the 3334-Systems Programming class coded in the Rust language.

The functionality of this project is to fetch the prices of Bitcoin, Ethereum and SP500 using an online http API.

The program should create 3 different txt files, each with the name of the data they represent, and the program will fetch the prices every 10 seconds to input it into the txt file.

If the fetching process fails, an error message will be displayed on the terminal and the previous value will be kept.

Also, since this is a project, it must be noted that I could not find a free API to fetch the SP500 prices, so the price is simulated using the tether cryptocoin as a replacement, as explained in the comments of the code.