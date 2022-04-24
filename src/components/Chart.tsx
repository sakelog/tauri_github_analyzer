import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Legend,
} from 'chart.js';
import { Line } from 'react-chartjs-2';

import { config } from 'components/config';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Legend
);

const Chart = ({
  labels,
  counts,
  mode,
}: {
  labels: string[];
  counts: number[];
  mode: 'views' | 'clones';
}) => {
  const options = {
    responsive: true,
    scales: {},
    plugins: {
      legend: {
        display: false,
      },
    },
  };

  const legendLabel = mode;
  const themeColor =
    mode === 'views'
      ? config.themeColor.views
      : config.themeColor.clones;

  const data = {
    labels,
    datasets: [
      {
        label: legendLabel,
        data: counts,
        borderColor: themeColor,
        backgroundColor: themeColor,
      },
    ],
  };

  return <Line options={options} data={data} />;
};

export default Chart;
