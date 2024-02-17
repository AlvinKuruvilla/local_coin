import pandas as pd
df = pd.read_csv("raw_transactions.csv")
df = df.dropna()
df.rename(columns={"inputs.addresses": "inputs_addresses", "outputs.addresses": "outputs_addresses", "outputs.value": "outputs_value"}, inplace=True)
print(df)
df.to_csv("transactions.csv", index=False)