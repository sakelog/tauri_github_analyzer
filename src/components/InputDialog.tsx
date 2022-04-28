import { useRef } from 'react';

// component
import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalFooter,
  Input,
  Button,
  Text,
} from '@chakra-ui/react';

// redux
import { useSelector, useDispatch } from 'react-redux';
import { RootState, AppDispatch } from 'redux/store';
import {
  setTmpPersonalToken,
  setTokenSubmitted,
} from 'redux/lib/slice';

// Main
type PropTypes = {
  isOpen: boolean;
  onClose: () => void;
};

const InputDialog = (props: PropTypes) => {
  const { isOpen, onClose } = props;

  // redux
  const errorMessage = useSelector<RootState>(
    (state) => state.mainState.errorMessage
  ) as string;

  const dispatch = useDispatch<AppDispatch>();

  const inputElement = useRef<HTMLInputElement>(null);
  const onClickButton = () => {
    dispatch(
      setTmpPersonalToken(inputElement.current?.value)
    );
    dispatch(setTokenSubmitted(true));
    onClose();
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>
          Input GitHub personal token.
        </ModalHeader>
        <ModalBody>
          <Text color="red" my={2}>
            {errorMessage}
          </Text>
          <Text fontStyle="italic">
            Please enter token.
          </Text>
          <Text fontStyle="italic">
            tokenを入力してください。
          </Text>
          <Input
            type="text"
            ref={inputElement}
            autoComplete="off"
            my={2}
          />
        </ModalBody>
        <ModalFooter>
          <Button type="submit" onClick={onClickButton}>
            Submit
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export default InputDialog;
