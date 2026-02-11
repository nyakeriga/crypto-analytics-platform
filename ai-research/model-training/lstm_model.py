import pandas as pd
import numpy as np
from sklearn.preprocessing import StandardScaler, LabelEncoder
from sklearn.model_selection import train_test_split
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import LSTM, Dense, Dropout
from tensorflow.keras.utils import to_categorical
import joblib

class LSTMModel:
    def __init__(self, seq_len=50, epochs=10, batch_size=32):
        self.seq_len = seq_len
        self.epochs = epochs
        self.batch_size = batch_size
        self.model = None
        self.scaler = StandardScaler()
        self.label_encoder = LabelEncoder()

    def load_data(self, filepath):
        self.data = pd.read_csv(filepath)
        # Assume columns: feature1, feature2, ..., signal
        feature_cols = [col for col in self.data.columns if col != 'signal']
        self.features = self.data[feature_cols].values
        self.targets = self.data['signal'].values

    def preprocess(self):
        # Scale features
        self.features_scaled = self.scaler.fit_transform(self.features)
        # Encode targets
        self.targets_encoded = self.label_encoder.fit_transform(self.targets)
        self.targets_categorical = to_categorical(self.targets_encoded)
        # Create sequences
        self.X, self.y = self.create_sequences(self.features_scaled, self.targets_categorical)

    def create_sequences(self, features, targets):
        X, y = [], []
        for i in range(len(features) - self.seq_len):
            X.append(features[i:i+self.seq_len])
            y.append(targets[i+self.seq_len])
        return np.array(X), np.array(y)

    def build_model(self, input_shape, num_classes):
        self.model = Sequential()
        self.model.add(LSTM(50, return_sequences=True, input_shape=input_shape))
        self.model.add(Dropout(0.2))
        self.model.add(LSTM(50))
        self.model.add(Dropout(0.2))
        self.model.add(Dense(num_classes, activation='softmax'))
        self.model.compile(optimizer='adam', loss='categorical_crossentropy', metrics=['accuracy'])

    def train(self, filepath, model_save_path='lstm_model.h5', scaler_save_path='scaler.pkl'):
        self.load_data(filepath)
        self.preprocess()
        input_shape = (self.seq_len, self.features.shape[1])
        num_classes = self.targets_categorical.shape[1]
        self.build_model(input_shape, num_classes)
        X_train, X_test, y_train, y_test = train_test_split(self.X, self.y, test_size=0.2, random_state=42)
        self.model.fit(X_train, y_train, epochs=self.epochs, batch_size=self.batch_size, validation_data=(X_test, y_test))
        self.save_model(model_save_path, scaler_save_path)

    def save_model(self, model_path, scaler_path):
        self.model.save(model_path)
        joblib.dump(self.scaler, scaler_path)
        joblib.dump(self.label_encoder, 'label_encoder.pkl')

if __name__ == "__main__":
    model = LSTMModel()
    model.train('data.csv')  # Assume data.csv is in the directory