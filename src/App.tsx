import { useState, useEffect } from 'react';

// lib
import { invoke } from '@tauri-apps/api/tauri';
import { sortInfoList } from 'lib/util/sortInfoList';

// component
import {
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  Center,
  Spinner,
} from '@chakra-ui/react';
import MyPanel from 'components/MyPanel';

// Main
const App = () => {
  const [trafficResults, setTrafficResults] = useState<
    Array<Github.RepoInfo>
  >([]);

  useEffect(() => {
    const onGetTraffics = async () => {
      const results = (await invoke(
        'fetch_repo_info'
      )) as Array<Github.RepoInfo>;
      setTrafficResults(results);
    };
    onGetTraffics();
  }, []);

  return (
    <Tabs>
      <TabList>
        <Tab>Views</Tab>
        <Tab>Clones</Tab>
      </TabList>
      {trafficResults.length === 0 && (
        <Center h="sm">
          <Spinner size="xl" color="gray.400" />
        </Center>
      )}
      <TabPanels>
        <TabPanel>
          <MyPanel
            trafficItems={sortInfoList(
              trafficResults,
              'views'
            )}
            mode="views"
          />
        </TabPanel>
        <TabPanel>
          <MyPanel
            trafficItems={sortInfoList(
              trafficResults,
              'clones'
            )}
            mode="clones"
          />
        </TabPanel>
      </TabPanels>
    </Tabs>
  );
};

export default App;
