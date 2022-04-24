import { formatDate } from 'lib/util/formatDate';

const sortByTimeStamp = (
  a: Github.Traffic,
  b: Github.Traffic
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
  targetArray: Array<Github.Traffic>
) => {
  targetArray.sort(sortByTimeStamp);
  const slicedArray = targetArray.slice(-5);

  let labels: string[] = [];
  let counts: number[] = [];

  slicedArray.map((item) => {
    labels.push(formatDate(item.timestamp));
    counts.push(item.count);
  });

  return {
    labels,
    counts,
  };
};
