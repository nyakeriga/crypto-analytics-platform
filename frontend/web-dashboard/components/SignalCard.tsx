import React from 'react';

interface Signal {
  id: string;
  symbol: string;
  type: 'buy' | 'sell';
  price: number;
  timestamp: string;
}

interface SignalCardProps {
  signal?: Signal;
}

const SignalCard: React.FC<SignalCardProps> = ({ signal }) => {
  if (!signal) {
    return <div>No signal data</div>;
  }

  return (
    <div style={{ border: '1px solid #ccc', padding: '10px', margin: '10px' }}>
      <h3>{signal.symbol}</h3>
      <p>Type: {signal.type}</p>
      <p>Price: {signal.price}</p>
      <p>Time: {signal.timestamp}</p>
    </div>
  );
};

export default SignalCard;