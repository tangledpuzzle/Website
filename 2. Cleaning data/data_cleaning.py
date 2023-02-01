import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import os

#-----------Cleaning 'data analyst' data-----------
def cleaning_da():
    data = pd.read_csv('data analyst.csv')
    data.drop_duplicates(inplace=True)

    indexes = [index for index, row in data.iterrows()
               if 'data scientist' in row['job title'].lower() or 'data engineering' in row['job title'].lower()]
    data.drop(index=indexes, inplace=True)

    # Giving job category and unique job ID
    data.insert(0, 'Job_ID', range(1, len(data) + 1))
    data[['temp', 'category']] = ['DA ', 'Data Analyst']
    data['Job_ID'] = data['temp'] + data['Job_ID'].astype(str)
    data.drop(columns = ['temp'], inplace=True)

    data = data.reset_index(drop=True)

    return data

#-----------Cleaning 'data scientist' data-----------
def cleaning_ds():
    data = pd.read_csv('data scientist.csv')
    data.drop_duplicates(inplace=True)

    indexes = [index for index, row in data.iterrows()
               if 'data engineer' in row['job title'].lower() or 'analyst' in row['job title'].lower()]
    data.drop(index=indexes, inplace=True)

    # Giving job category and unique job ID
    data.insert(0, 'Job_ID', range(1, len(data) + 1))
    data[['temp', 'category']] = ['DS ', 'Data Scientist']
    data['Job_ID'] = data['temp'] + data['Job_ID'].astype(str)
    data.drop(columns=['temp'], inplace=True)

    data = data.reset_index(drop=True)

    return data

#-----------Organizing and cleaning salary and experience details-----------
def salary_data(data):
    data = data[data['salary'] != 'Not Disclosed'] # Removing 'Not disclosed' rows

    # we have experience and salary as (x-y) and (a-b), so we spliting it as low and high

    # Getting low salary and experience details (x) and (a)
    data_low = data[['Job_ID', 'experience', 'salary']].copy()

    data_low['salary'] = data_low['salary'].apply(lambda x: int(x[2:-5].replace(',', '').split(' - ')[0]))
    data_low['experience'] = data_low['experience'].apply(lambda x: int(x[:-5].split(' - ')[0]))

    # Getting high salary and experience details (y) and (b)
    data_high = data[['Job_ID', 'experience', 'salary']].copy()

    data_high['salary'] = data_high['salary'].apply(lambda x: int(x[2:-5].replace(',', '').split(' - ')[-1].split(' ')[0]))
    data_high['experience'] = data_high['experience'].apply(lambda x: int(x[:-5].split(' - ')[-1]))

    data = pd.concat([data_low, data_high]) # combining both details
    data.drop_duplicates(inplace=True)

    # checking for outliers using matplotlib
    '''x = data['experience']
    y = data['salary']

    plt.scatter(x, y)

    plt.xlabel('experience')
    plt.ylabel('salary')
    plt.show()'''

    # Finding out the outliers
    low_sal, high_sal = find_outlier(data, 'salary')
    low_exp, high_exp = find_outlier(data, 'experience')

    # Removing the outliers...
    data = data[(data['salary'] > low_sal) & (data['salary'] < high_sal) & 
    			(data['experience'] > low_exp) & (data['experience'] < high_exp)]

    return data

#-----------To find the outlier in a given column-----------
def find_outlier(data, column):
	column = data[column]

	q1 = np.percentile(column, 25)
	q3 = np.percentile(column, 75)

	iqr = q3 - q1

	low = q1 - 1.5 * iqr
	high = q3 + 1.5 * iqr

	# print(low, high)

	return low, high

#-----------Organizing and cleaning location details-----------
def location_data(data):
    data = data[['Job_ID', 'location']].copy()
    data['location'] = data['location'].apply(lambda x : x[:-9] if 'View More' in x else x) # Removing the 'view more'

    data['location'] = data['location'].str.split(',') # splitting all location as list
    data = data.explode('location') # exploding by location
    data = data.reset_index(drop=True)

    # Iterating and cleaning location data
    for index, row in data.iterrows():
        if 'hiring office' in row['location'].lower():
            row['location'] = row['location'].split('in')[-1]
        if '(' in row['location'].lower():
            row['location'] = row['location'].split('(')[0]

    data['location'] = data['location'].apply(lambda x : x.split('/')[0]).apply(lambda x : 'other'
                        if find_other(x) else x).apply(lambda x : x.lower())

    data = data[data['location'] != 'other']

    return data

#-----------Helping to find cities outside India-----------
def find_other(city):
    cities = ['united', 'usa', 'not specified', 'ny', 'india', 'australia', 'chicago', 'africa', 'new york']

    for i in cities:
        if i in city.lower():
            return True
    return False

#-----------Organizing education details-----------
def education_data(data):
    data = data[['Job_ID', 'education']].copy()
    data['education'] = data['education'].str.strip().str.replace('  ', ' ').str.split(' ')

    data = data.explode('education')
    return data

#-----------Organizing and cleaning skills data
def skills_data(data):
    data = data[['Job_ID', 'python', 'r', 'sql', 'excel', 'vba', 'powerbi',
                 'tableau', 'nosql', 'sas', 'git', 'matlab']].astype(str).copy()
    data['skills'] = data[['python', 'r', 'sql', 'excel', 'vba', 'powerbi', 'tableau', 'nosql', 'sas',
                           'git', 'matlab']].agg(','.join, axis=1).str.split(',')

    data = data[['Job_ID', 'skills']]
    data = data.explode('skills')
    data = data[data['skills'] != 'nan']

    return data


if __name__ == '__main__':
	# Cleaning and combining data
	data_analyst_df = cleaning_da()
	data_scientist_df = cleaning_ds()
	df = pd.concat([data_analyst_df, data_scientist_df])

	salary = salary_data(df)
	location = location_data(df)
	education = education_data(df)
	skill = skills_data(df)

	df_dict = {'raw': df, 'salary': salary, 'location': location, 'education': education, 'skill' : skill}

	if not os.path.exists('final_data'):
	    os.mkdir('final_data')

	# Saving the final datas in the sheets of an excel
	with pd.ExcelWriter('final_data/final_data.xlsx', engine='xlsxwriter') as writer:
	    for sheet_name in df_dict.keys():
	        df_dict[sheet_name].to_excel(writer, sheet_name=sheet_name, index=False)

	print('Cleaned and saved as "final_data".')


'''Index(['job title', 'company', 'experience', 'salary', 'location', 'education',
       'python', 'r', 'sql', 'excel', 'vba', 'powerbi', 'tableau', 'nosql',
       'sas', 'git', 'matlab'] 4072 rows'''