import pandas as pd
import numpy as np
from typing import List, Dict, Optional
import logging
from dataclasses import dataclass
from datetime import datetime

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class Trade:
    timestamp: datetime
    symbol: str
    side: str  # 'buy' or 'sell'
    quantity: float
    price: float
    commission: float = 0.0

@dataclass
class Position:
    symbol: str
    quantity: float
    avg_price: float
    unrealized_pnl: float = 0.0

class PaperTradingSimulator:
    def __init__(self, initial_balance: float = 10000.0, commission_rate: float = 0.001):
        self.initial_balance = initial_balance
        self.balance = initial_balance
        self.commission_rate = commission_rate
        self.positions: Dict[str, Position] = {}
        self.trades: List[Trade] = []
        self.portfolio_history: List[Dict] = []

    def execute_signal(self, signal: Dict, market_data: pd.DataFrame) -> Optional[Trade]:
        """
        Execute a trading signal
        signal: {'symbol': str, 'side': str, 'quantity': float, 'timestamp': datetime}
        """
        try:
            symbol = signal['symbol']
            side = signal['side']
            quantity = signal['quantity']
            timestamp = signal['timestamp']

            # Get current price from market data
            current_data = market_data[market_data['timestamp'] <= timestamp]
            if current_data.empty:
                logger.warning(f"No market data available for {symbol} at {timestamp}")
                return None

            current_price = current_data.iloc[-1]['close']

            # Calculate commission
            commission = current_price * quantity * self.commission_rate

            # Check if we have enough balance for buy
            if side == 'buy':
                cost = (current_price * quantity) + commission
                if cost > self.balance:
                    logger.warning(f"Insufficient balance for buy: {cost} > {self.balance}")
                    return None
                self.balance -= cost
            else:  # sell
                if symbol not in self.positions or self.positions[symbol].quantity < quantity:
                    logger.warning(f"Insufficient position for sell: {symbol}")
                    return None
                proceeds = (current_price * quantity) - commission
                self.balance += proceeds

            # Update positions
            self._update_position(symbol, side, quantity, current_price)

            # Record trade
            trade = Trade(
                timestamp=timestamp,
                symbol=symbol,
                side=side,
                quantity=quantity,
                price=current_price,
                commission=commission
            )
            self.trades.append(trade)

            logger.info(f"Executed {side} {quantity} {symbol} at {current_price}")
            return trade

        except Exception as e:
            logger.error(f"Error executing signal: {str(e)}")
            return None

    def _update_position(self, symbol: str, side: str, quantity: float, price: float):
        if symbol not in self.positions:
            self.positions[symbol] = Position(symbol, 0, 0)

        position = self.positions[symbol]

        if side == 'buy':
            total_cost = position.avg_price * position.quantity + price * quantity
            position.quantity += quantity
            position.avg_price = total_cost / position.quantity if position.quantity > 0 else 0
        else:  # sell
            position.quantity -= quantity
            if position.quantity <= 0:
                position.avg_price = 0

        # Update unrealized P&L (simplified, assuming current price)
        position.unrealized_pnl = (price - position.avg_price) * position.quantity

    def get_portfolio_value(self, current_prices: Dict[str, float]) -> float:
        total_value = self.balance
        for symbol, position in self.positions.items():
            if symbol in current_prices:
                total_value += position.quantity * current_prices[symbol]
        return total_value

    def get_performance_metrics(self) -> Dict:
        if not self.trades:
            return {'total_return': 0, 'win_rate': 0, 'total_trades': 0}

        total_return = (self.balance - self.initial_balance) / self.initial_balance

        winning_trades = sum(1 for trade in self.trades if trade.side == 'sell')  # Simplified
        win_rate = winning_trades / len(self.trades) if self.trades else 0

        return {
            'total_return': total_return,
            'win_rate': win_rate,
            'total_trades': len(self.trades),
            'current_balance': self.balance
        }

    def run_backtest(self, signals: List[Dict], market_data: pd.DataFrame) -> Dict:
        """
        Run backtest with list of signals and market data
        """
        logger.info("Starting backtest...")

        for signal in signals:
            self.execute_signal(signal, market_data)

        # Record final portfolio state
        final_prices = {row['symbol']: row['close'] for _, row in market_data.iterrows()}
        final_value = self.get_portfolio_value(final_prices)

        metrics = self.get_performance_metrics()
        metrics['final_portfolio_value'] = final_value

        logger.info(f"Backtest completed. Final value: {final_value}")
        return metrics

# Example usage
def main():
    simulator = PaperTradingSimulator(initial_balance=10000.0)

    # Sample market data
    market_data = pd.DataFrame({
        'timestamp': pd.date_range('2023-01-01', periods=100, freq='1H'),
        'symbol': 'BTCUSDT',
        'close': np.random.uniform(40000, 50000, 100)
    })

    # Sample signals
    signals = [
        {'symbol': 'BTCUSDT', 'side': 'buy', 'quantity': 0.1, 'timestamp': pd.Timestamp('2023-01-01 10:00:00')},
        {'symbol': 'BTCUSDT', 'side': 'sell', 'quantity': 0.1, 'timestamp': pd.Timestamp('2023-01-02 10:00:00')},
    ]

    results = simulator.run_backtest(signals, market_data)
    print("Backtest Results:", results)

if __name__ == "__main__":
    main()