# Analysis of data scientist and analyst job in India #

  In this project, data scientist and data analyst job data from the Naukri job portal in India was collected through python web scraping, cleaned and analyzed using python data manupilation tools like pandas, numpy and visualized using PowerBI. This analysis showed a high demand for data scientists and data analysts in India, with the tech and finance industries having the highest demand, particularly in cities such as Mumbai, Bengaluru, and Delhi. The analysis and visualization provided insights into the job market for these roles, including salary ranges and required skills.
  
  The steps I have done in this projects are:
    * **Collecting Data**
    * **Data Wrangling**
    * **Visualaizing data**
    
    
### Collecting Data
  I collected data from Naukri job Portals. Naukri is a popular job portal in India, offering a vast collection of job postings in various industries. In this project I collect data scientist and data analyst job data from Naukri in order to understand the job market trends and demands in India. I used web scraping techniques, which involves automatically extracting information from websites. 
  
  The web scraping was done using Python Selenium and BeautifulSoup. You can look at my code which I was saved as **[data_scraping.py](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/data_scraping.py)**. When running this code it asks two inputs - first the job title need to search and the total number of pages needed to be scraped. I made this code as available to scrape any type of given job title from naukri website with the required number of pages. For my project I scraped [data analyst](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/data%20analyst.csv) and [data scientist](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/data%20scientist.csv). I also used a [json](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/1.%20Scraping%20data/helper.json) file. It contains the keyword which is helps the code to search skill and education details.
  
  
### Data Wrangling
  Data wrangling and cleaning is an essential step in the data analysis process to ensure that the data being analyzed is accurate, consistent, and free of errors. The tasks in this step includes removing duplicates, filling the missing values, transforming data and converting data types. For this analysis I like to separate data as education, salary and experience, location, skills because some of the jobs contain more than one of these details. For processing this step I used python **Pandas**.
  
  Pandas is a powerful library for data cleaning and manipulation in Python. Its intuitive interface and wide range of functions for data cleaning, makes it an essential tool for any data analyst or scientist. I loaded the CSV files into pandas dataframes, cleaned and then combined them. Then I seperated my data as per my requirement needed for analysis. [click here](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/2.%20Cleaning%20data/data_cleaning.py) to see the python code. The transformed datas were saved into the sheets of the [fina data](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/2.%20Cleaning%20data/final_data/final_data.xlsx) excel file.
  
  
### Visualaizing data
   Visualisation of data brings the effective communication of findings and insights. **Power BI** is a data visualization and business intelligence tool from Microsoft that provides a wide range of features for creating interactive and engaging visualizations. Power BI also provides features for data exploration and discovery. This includes interactive dashboards, drill-through options, and the ability to create custom calculations using DAX.
    
   I imported the final data into powerbi after transforming the datas using power query. Calculated some measures using DAX. Visualized graphs for average salary by experience and proportion of skills, pie chart for level of education required, Cards to find out the preference of work from home or work from office jobs. For svisualaization look at my [visualization file](https://github.com/Poova53/Analysis-of-data-scientist-and-analyst-job-in-India/blob/d7f3206b73af38bfc99419ecf163ad53167db6b9/3.%20Visualizing%20data/analysis%20of%20data%20analyst%20and%20data%20scientist%20in%20India.pbix).
    
    
## Result ##
   * From the visualization of skills graph, it has been noted that **Python** is the most asked skill across the jobs followed by SQL, Excel, etc,.
   * The average salary is significantly increasing by experience.
   * Bachelor and Master degree is preferred in the most of the companies.
   * Nearly 96% of companies preferred to work from office.
