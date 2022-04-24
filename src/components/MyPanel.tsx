// lib
import { convertToChartData } from 'lib/util/convertToChartData';
import { sortInfoList } from 'lib/util/sortInfoList';

// component
import {
  Grid,
  GridItem,
  Flex,
  Heading,
  Text,
  Center,
  Circle,
  Link,
  Tooltip,
} from '@chakra-ui/react';
import Chart from 'components/Chart';

// config
import { config } from 'components/config';

type PropTypes = {
  trafficItems: Github.RepoInfo[];
  mode: 'views' | 'clones';
};

// Main
const MyPanel = (props: PropTypes) => {
  const { trafficItems, mode } = props;
  const sortedTrafficItems = sortInfoList(
    trafficItems,
    mode
  );

  return (
    <Grid
      templateColumns={{
        base: 'repeat(1, 1fr)',
        sm: 'repeat(2, 1fr)',
        md: 'repeat(3, 1fr)',
      }}
      gap={6}
      p={4}
    >
      {sortedTrafficItems.map((item) => {
        const { name, url } = item;

        const targetInfo =
          mode === 'views'
            ? item.views_info
            : item.clones_info;

        const chartData = convertToChartData(
          targetInfo.items
        );
        const themeColor =
          mode === 'views'
            ? config.themeColor.views
            : config.themeColor.clones;
        return (
          <GridItem
            overflow="hidden"
            border="2px"
            borderColor="gray.400"
            p={2}
            key={name}
          >
            <Flex alignItems="center" p={2}>
              <Link
                href={url}
                display="block"
                flex="1"
                overflow="hidden"
                color="blue.400"
                isExternal
              >
                <Tooltip label={name}>
                  <Heading fontSize="2xl" isTruncated>
                    {name}
                  </Heading>
                </Tooltip>
              </Link>
              <Circle size="12" bg={themeColor}>
                <Center>
                  <Text
                    fontWeight="bold"
                    fontSize="lg"
                    color="white"
                  >
                    {targetInfo.count}
                  </Text>
                </Center>
              </Circle>
            </Flex>
            <Chart
              labels={chartData.labels}
              counts={chartData.counts}
              mode={mode}
            />
          </GridItem>
        );
      })}
    </Grid>
  );
};

export default MyPanel;
