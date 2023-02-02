# Analysis of data scientist and analyst job in India #

  In this project, job data for data scientists and data analysts was scraped from the Naukri job portal in India using Python. The data was then cleaned, processed, and analysed using Python data management tools like pandas and numpy, and then displayed using PowerBI. The results of this investigation demonstrated that there is a significant need for data scientists and analysts in India, with the IT and financial sectors having the largest need, particularly in places like Mumbai, Bengaluru, and Delhi. With regard to wage ranges and necessary abilities, the analysis and visualisation offered insights into the job market for these roles.
  
  The steps I have done in this projects are:
   * **Collecting Data**
   * **Data Wrangling**
   * **Visualaizing data**
    
    
### Collecting Data
  I gathered the data from Naukri job portals. With a large selection of job postings across numerous industries, Naukri is a well-known job platform in India. To understand the requirements and trends of the Indian job market, I am gathering data scientist and data analyst job data from Naukri for this study. In order to automatically extract information from websites, I utilized web scraping techniques. 
  
  Python Selenium and BeautifulSoup were used to perform the web scraping. You may view my code, which I saved as **[data_scraping.py](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/data_scraping.py)**. When running this code, it requires two inputs: the job title to search for plus the total amount of pages to scrape. I provided this code to allow users to scrape any specified job title out from required amount of pages on the naukri website. I scraped [data analyst](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/data%20analyst.csv) and [data scientist](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/data%20scientist.csv) jobs for my project. I also used a [json](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/helper.json) file. It contains some keywords that the code uses to look for information about education and skills..
  
  
### Data Wrangling
  Data wrangling and cleaning are crucial steps in the data analysis process to guarantee that the data being analysed is accurate, consistent, and error-free. This process involves tasks like identifying and removing duplicates, adding missing values, manipulating data and data types. Considering the fact that certain employment involve over many elements, I prefer to segregate the data by degree, income and experience, geography, and skills for this research. I used the Python Pandas library to process this phase.
  
  Pandas is a powerful library for data cleaning and manipulation in Python. Its intuitive interface and wide range of functions for data cleaning, makes it an essential tool for any data analyst or scientist. I loaded the CSV files into pandas dataframes, cleaned and then combined them. Then I seperated my data as per my requirement needed for analysis. [click here](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/2.%20Cleaning%20data/data_cleaning.py) to see the python code. The transformed datas were saved into the sheets of the [fina data](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/2.%20Cleaning%20data/final_data/final_data.xlsx) excel file.
  
  
### Visualaizing data
   The efficient sharing of thoughts and conclusions is made possible by visualization techniques. Microsoft's **Power BI** is a tool for data visualisation and business intelligence that offers a variety of tools for building interactive and interesting visualisations. Data exploration and discovery tools are also available in Power BI. This features drill-through capabilities, interactive dashboards, and DAX support for creating different calculations measures.
    
   I imported the final data into powerbi after transforming the datas using power query. Calculated some measures using DAX. Visualized graphs for average salary by experience and proportion of skills, pie chart for level of education required, Cards to find out the preference of work from home or work from office jobs. For svisualaization look at my [visualization file](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/3.%20Visualizing%20data/analysis%20of%20data%20analyst%20and%20data%20scientist%20in%20India.pbix).
    
    
## Result ##
  * The most commonly requested skill across all industries, according to the skills graph visualization, is Python, which is followed by SQL, Excel, etc. 
  * With experience, the average wage rises dramatically. 
  * The majority of employers preferred applicants with bachelor's and master's degrees. 
  * Nearly 96% of businesses chose non-remote workers.
