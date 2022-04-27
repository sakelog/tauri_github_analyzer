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
  useDisclosure,
} from '@chakra-ui/react';
import MyPanel from 'components/MyPanel';
import InputDaialog from 'components/InputDialog';

// redux
import { useDispatch, useSelector } from 'react-redux';
import { AppDispatch, RootState } from 'redux/store';
import {
  setTmpPersonalToken,
  setTokenSubmitted,
} from 'redux/lib/slice';

// Main
const App = () => {
  const [trafficResults, setTrafficResults] = useState<
    Array<Github.RepoInfo>
  >([]);

  const { isOpen, onOpen, onClose } = useDisclosure();

  // redux
  const tmpToken = useSelector<RootState>(
    (state) => state.mainState.tmpPersonalToken
  ) as string;
  const tokenSubmitted = useSelector<RootState>(
    (state) => state.mainState.tokenSubmitted
  ) as boolean;

  const dispatch = useDispatch<AppDispatch>();

  useEffect(() => {
    const onCheckExistFile = async () => {
      const checkResult = await invoke<boolean>(
        'check_exist_file'
      );
      let resultJson: Github.RepoInfo[] = [];
      if (checkResult) {
        resultJson = await invoke<Github.RepoInfo[]>(
          'fetch_repo_info'
        );
      } else {
        if (tokenSubmitted) {
          resultJson = await invoke<Github.RepoInfo[]>(
            'fetch_repo_info',
            { newPersonalToken: tmpToken }
          );
        } else {
          onOpen();
        }
        dispatch(setTokenSubmitted(false));
        dispatch(setTmpPersonalToken(''));
      }

      setTrafficResults(resultJson);
    };

    onCheckExistFile();
  }, [tokenSubmitted]);

  return (
    <>
      <InputDaialog isOpen={isOpen} onClose={onClose} />
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
    </>
  );
};

export default App;
