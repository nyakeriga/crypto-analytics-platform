import React from 'react';

interface Indicator {
  name: string;
  value: number;
}

interface IndicatorPanelProps {
  indicators?: Indicator[];
}

const IndicatorPanel: React.FC<IndicatorPanelProps> = ({ indicators = [] }) => {
  return (
    <div style={{ border: '1px solid #ccc', padding: '10px', margin: '10px' }}>
      <h3>Indicators</h3>
      {indicators.length === 0 ? (
        <p>No indicators</p>
      ) : (
        <ul>
          {indicators.map((indicator, index) => (
            <li key={index}>
              {indicator.name}: {indicator.value}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default IndicatorPanel;