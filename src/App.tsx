import { useState, useEffect } from 'react';

// lib
import { invoke } from '@tauri-apps/api/tauri';

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
    // const newPersonalToken =
    //   process.env.REACT_APP_GITHUB_PERSONAL_ACCESS_TOKEN;

    const onGetTraffics = async () => {
      const results = await invoke<Github.RepoInfo[]>(
        'fetch_repo_info'
      );
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
            trafficItems={trafficResults}
            mode="views"
          />
        </TabPanel>
        <TabPanel>
          <MyPanel
            trafficItems={trafficResults}
            mode="clones"
          />
        </TabPanel>
      </TabPanels>
    </Tabs>
  );
};

export default App;
