import { useState, useEffect } from 'react';
import { fetchSignals } from '../services/api';

interface Signal {
  id: string;
  symbol: string;
  type: 'buy' | 'sell';
  price: number;
  timestamp: string;
}

const useSignals = () => {
  const [signals, setSignals] = useState<Signal[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadSignals = async () => {
      try {
        const data = await fetchSignals();
        setSignals(data);
      } catch (err) {
        setError('Failed to fetch signals');
      } finally {
        setLoading(false);
      }
    };

    loadSignals();
  }, []);

  return { signals, loading, error };
};

export default useSignals;