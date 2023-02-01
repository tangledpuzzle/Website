import csv
import json
import os
import time
from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from bs4 import BeautifulSoup
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from webdriver_manager.chrome import ChromeDriverManager


class WebScrape:
    def __init__(self):
        self.driver = None # creating a variable for driver
        self.job_title = input('Enter job title: ')  # get the job title we want to scrape
        self.total_page = int(input('\nHow many pages need to scrape: ')) # No. of pages need to scrape(each page contain 20 jobs)
        self.file_name = self.job_title + '.csv'  # data will be saved as this CSV file name 
        self.job_data = [] # first we store each job details in this list and then add it to CSV

        self.search_job() # Initializing web driver
        self.start_scrape() # Start scraping the page


	#-----------Initializing chrome web driver-----------
    def search_job(self):
        chrome_option = Options()
        chrome_option.add_argument('--ignore-certificate-errors')
        chrome_service = Service(ChromeDriverManager().install())

        self.driver = webdriver.Chrome(options=chrome_option, service=chrome_service)

        self.driver.get(r'https://www.naukri.com') # entering naukri website

        inp = self.driver.find_element(by=By.CLASS_NAME, value='keywordSugg') # Finding the search box
        inp = inp.find_element(by=By.CLASS_NAME, value='suggestor-input ')

        inp.send_keys(self.job_title) # entering the job title
        time.sleep(3)

        inp.send_keys(Keys.ENTER) 
        time.sleep(4)

        self.current_page_no = self.get_current_page_no() # getting current page no
        self.current_page_url = self.driver.current_url   # getting current url

    #-----------Start scraping the page-----------
    def start_scrape(self):
        while self.current_page_no <= self.total_page:  # Looping until we get all the pages
            print(f"Started scraping page no: {self.current_page_no}")

            self.navigate_links() # This method scrape all the jobs in each page

            print(f'Completed scraping links in page no {self.current_page_no}\n\n')

            # Checking if we completed the targeted page
            if self.current_page_no == self.total_page: 
                print(f'Scraped all pages\n')

                with open(self.file_name, 'r') as file:
                    reader = csv.reader(file)
                    data = list(reader)
                print(f'{len(data)-1} {self.job_title} jobs data added')
                break

            self.go_to_next_page() # This method moves to next page


    #-----------Scraping all jobs in the page-----------
    def navigate_links(self):
        links = self.get_job_links() # It returns the list of all job links in the page

        # Iterating through each links
        for index, link in enumerate(links):
            print(f'scraping link: {index+1}')

            self.driver.get(link)
            self.job_data = [] # making it has empty list to get next job data
            time.sleep(0.5)

            try:
                self.scrape_job() # collecting data from the job links
                self.add_to_csv() # appending it to CSV

            except AttributeError: # if it contains different HTML content
                print("OOPS! it contain different content\nSkipping this link")

            except Exception as e: 
                print(f"unexpected error \n{e} \nSkipping this link")

    #-----------Collecting data from the job link-----------
    def scrape_job(self):
        soup = self.get_page_content() # it returns the page content

        self.job_heading(soup)  # getting data from the job heading
        self.job_description(soup) # getting data from the job description

    #-----------Getting job links in each page-----------
    def get_job_links(self):
        soup = self.get_page_content()
        a_tag = soup.find_all('a', {'class': 'title ellipsis'})
        links = [a['href'] for a in a_tag]
        return links

    #-----------Getting the current page no-----------
    def get_current_page_no(self):
        soup = self.get_page_content()
        div = soup.find('div', {'class': 'pagination lastCompMark'})
        current_page = div.find('a', {'class': 'selected'})
        return int(current_page.text.strip())

    #-----------Moving to next page-----------
    def go_to_next_page(self):
        self.driver.get(self.current_page_url) # going to current page using the current_page_url
        time.sleep(4)

        print('Moving to next page\n')

        next_link = self.driver.find_element(by=By.LINK_TEXT, value='Next') # finding the next page link
        next_link.click()
        time.sleep(4)

        # Saving the current page no and url
        self.current_page_no = self.get_current_page_no()
        self.current_page_url = self.driver.current_url

    #-----------Getting the page content using BeautifulSoup-----------
    def get_page_content(self):
        page_source = self.driver.page_source

        soup = BeautifulSoup(page_source, 'lxml')
        return soup

    #-----------Scraping all data in job headings-----------
    def job_heading(self, content):
        section = content.find('section', {'class': 'jd-header'})

        job_title = section.find('h1', {'class': 'jd-header-title'}).text
        company_name = section.find('a', {'class': 'pad-rt-8'}).text
        experience = section.find('div', {'class': 'exp'}).text.strip()
        salary = section.find('div', {'class': 'salary'}).text.strip()
        location = section.find('div', {'class': 'loc'}).text.strip()

        self.job_data.extend([job_title, company_name, experience, salary, location]) # Appending it to the list

    #-----------Scraping all data in job descriptions-----------
    def job_description(self, content):
        section = content.find('section', {'class': 'job-desc'})
        data = self.helper()

        # find education, python, python, sql, excel, vba, powerbi, tableau, R, SAS, git, NoSQL
        education = self.find_education(section, data)
        skill_text = self.get_skill_content(section)
        python = self.search_skill(skill_text, data, 'python')
        sql = self.search_skill(skill_text, data, 'sql')
        nosql = self.search_skill(skill_text, data, 'nosql')
        r = self.search_skill(skill_text, data, 'r')
        excel = self.search_skill(skill_text, data, 'excel')
        tableau = self.search_skill(skill_text, data, 'tableau')
        powerbi = self.search_skill(skill_text, data, 'powerbi')
        vba = self.search_skill(skill_text, data, 'vba')
        git = self.search_skill(skill_text, data, 'git')
        sas = self.search_skill(skill_text, data, 'sas')
        matlab = self.search_skill(skill_text, data, 'matlab')

        self.job_data.extend([education, python, r, sql, excel, vba, powerbi, tableau, nosql, sas, git, matlab]) # Appending it to list

    #-----------Education details-----------
    def find_education(self, section, data):
        education = ''
        try:
            div = section.find('div', {'class': 'education'})
            details = div.find_all('div', {'class': 'details'})
            details = [detail.text.lower() for detail in details]
            details = ''.join(details)

            for i in data['bachelor']:
                if i in details:
                    education += 'bachelor '
                    break

            for j in data['master']:
                if j in details:
                    education += ' master '
                    break

            for k in data['phd']:
                if k in details:
                    education += ' phd '
                    break

        except:
            education = ''

        return education

    #-----------Searching a given skill(text) in the content using data(json)-----------
    def search_skill(self, content, data, text):
        for i in data[text]:
            if i in content:
                return text
        return ''

    #-----------Getting all job description to find out the skills-----------
    def get_skill_content(self, section):
        job_description = section.find('div', {'class': 'dang-inner-html'}).text.lower()
        key_skill = section.find('div', {'class': 'key-skill'}).text.lower()

        return key_skill + job_description

    #-----------Return a json which contains keywords to search skill and education-----------
    def helper(self):
        with open('helper.json') as f:
            data = json.load(f)
        return data

    #-----------Adding the data to CSV-----------
    def add_to_csv(self):
        files = os.listdir()
        
        # checking if file_name exist
        if self.file_name in files:
            with open(self.file_name, 'a', newline='', encoding='utf-8') as file:
                csv_writer = csv.writer(file)
                csv_writer.writerow(self.job_data)
        else:
            header = ['job title', 'company', 'experience', 'salary', 'location', 'education', 'python', 'r', 'sql',
                      'excel', 'vba', 'powerbi', 'tableau', 'nosql', 'sas', 'git', 'matlab']
            with open(self.file_name, 'w', newline='', encoding='utf-8') as file:
                csv_writer = csv.writer(file)
                csv_writer.writerow(header)
                csv_writer.writerow(self.job_data)
        print('data added to csv\n')


if __name__ == '__main__':
    scrape_job = WebScrape
    scrape_job()
    
