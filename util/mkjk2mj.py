import argparse
import os
import pandas as pd

# Parse command line arguments
parser = argparse.ArgumentParser()
parser.add_argument('input_file', help='path to the input CSV file')
args = parser.parse_args()

# Read the CSV file and select only the necessary columns
data = pd.read_csv(args.input_file, usecols=['対応するUCS', '住基ネット統一文字コード']).dropna(subset=['対応するUCS', '住基ネット統一文字コード'])
data['対応するUCS'] = data['対応するUCS'].apply(lambda x: chr(int(x[2:], 16)))
data['住基ネット統一文字コード'] = data['住基ネット統一文字コード'].apply(lambda x: chr(int(x[2:], 16)))
data = data.rename(columns={'対応するUCS': 'MJ', '住基ネット統一文字コード': '住基'})
juki_mj_mapping = data[['住基', 'MJ']]

# Save the mapping to a new CSV file in the same directory as the input file
output_path = os.path.join(os.path.dirname(args.input_file), 'juki2mj.csv')
juki_mj_mapping.to_csv(output_path, index=False, encoding='utf-8')

# Preview the first few rows of the mapping
print(juki_mj_mapping.head())
