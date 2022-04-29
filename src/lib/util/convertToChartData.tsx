import { formatDate } from 'lib/util/formatDate';

const sortByTimeStamp = (
  a: GitHub.Traffic,
  b: GitHub.Traffic
) => {
  const timestampA = a.timestamp;
  const timestampB = b.timestamp;

  if (timestampA > timestampB) {
    return 1;
  }
  if (timestampA < timestampB) {
    return -1;
  }

  return 0;
};

export const convertToChartData = (
  targetArray: Array<GitHub.Traffic>
) => {
  targetArray.sort(sortByTimeStamp);
  const slicedArray = targetArray.slice(-5);

  const labels: string[] = [];
  const counts: number[] = [];

  // eslint-disable-next-line array-callback-return
  slicedArray.map((item) => {
    labels.push(formatDate(item.timestamp));
    counts.push(item.count);
  });

  return {
    labels,
    counts,
  };
};

export default { convertToChartData };
