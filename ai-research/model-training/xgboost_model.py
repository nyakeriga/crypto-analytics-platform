import pandas as pd
import numpy as np
from sklearn.preprocessing import StandardScaler, LabelEncoder
from sklearn.model_selection import train_test_split
from xgboost import XGBClassifier
import joblib

class XGBoostModel:
    def __init__(self):
        self.model = None
        self.scaler = StandardScaler()
        self.label_encoder = LabelEncoder()

    def load_data(self, filepath):
        self.data = pd.read_csv(filepath)
        feature_cols = [col for col in self.data.columns if col != 'signal']
        self.features = self.data[feature_cols].values
        self.targets = self.data['signal'].values

    def preprocess(self):
        self.features_scaled = self.scaler.fit_transform(self.features)
        self.targets_encoded = self.label_encoder.fit_transform(self.targets)

    def build_model(self):
        self.model = XGBClassifier(objective='multi:softprob', num_class=len(np.unique(self.targets_encoded)))

    def train(self, filepath, model_save_path='xgboost_model.pkl', scaler_save_path='scaler.pkl'):
        self.load_data(filepath)
        self.preprocess()
        self.build_model()
        X_train, X_test, y_train, y_test = train_test_split(self.features_scaled, self.targets_encoded, test_size=0.2, random_state=42)
        self.model.fit(X_train, y_train, eval_set=[(X_test, y_test)], early_stopping_rounds=10, verbose=True)
        self.save_model(model_save_path, scaler_save_path)

    def save_model(self, model_path, scaler_path):
        joblib.dump(self.model, model_path)
        joblib.dump(self.scaler, scaler_path)
        joblib.dump(self.label_encoder, 'label_encoder.pkl')

if __name__ == "__main__":
    model = XGBoostModel()
    model.train('data.csv')