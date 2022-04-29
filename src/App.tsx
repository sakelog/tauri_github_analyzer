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
import ModalError from 'components/ModalError';

// redux
import { useDispatch, useSelector } from 'react-redux';
import { AppDispatch, RootState } from 'redux/store';
import {
  setTmpPersonalToken,
  setTokenSubmitted,
  setErrorMessage,
  incrementCountTokenSend,
} from 'redux/lib/slice';

// Main
const App = () => {
  const [trafficResults, setTrafficResults] = useState<
    Array<GitHub.RepoInfo>
  >([]);

  const { isOpen, onOpen, onClose } = useDisclosure();

  // redux
  const tmpToken = useSelector<RootState>(
    (state) => state.mainState.tmpPersonalToken
  ) as string;
  const tokenSubmitted = useSelector<RootState>(
    (state) => state.mainState.tokenSubmitted
  ) as boolean;
  const countTokenSend = useSelector<RootState>(
    (state) => state.mainState.countTokenSend
  ) as number;

  const dispatch = useDispatch<AppDispatch>();

  useEffect(() => {
    const onCheckExistFile = async () => {
      const checkResult = await invoke<boolean>(
        'check_exist_file'
      );
      let resultJson: string | null = null;
      if (checkResult) {
        resultJson = await invoke<string>(
          'fetch_repo_info'
        ).catch(() => null);
        dispatch(incrementCountTokenSend());
      } else {
        if (tokenSubmitted) {
          resultJson = await invoke<string>(
            'fetch_repo_info',
            { newPersonalToken: tmpToken }
          ).catch(() => null);
          dispatch(incrementCountTokenSend());
        } else {
          onOpen();
        }
        dispatch(setTokenSubmitted(false));
        dispatch(setTmpPersonalToken(''));
      }

      let result: GitHub.RepoInfo[] = [];
      if (typeof resultJson === 'string') {
        result = JSON.parse(resultJson);
        setTrafficResults(result);
      } else {
        if (countTokenSend > 0) {
          dispatch(
            setErrorMessage(
              `token set error! : [ ${countTokenSend} ]`
            )
          );
        }
        await invoke<void>('delete_file');
      }
    };
    onCheckExistFile();
  }, [tokenSubmitted]);

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
      {countTokenSend > 5 ? (
        <ModalError
          msgTitle="Personal access token sending limit over!"
          isOpen
          onClose={onClose}
        />
      ) : (
        <InputDaialog isOpen={isOpen} onClose={onClose} />
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
