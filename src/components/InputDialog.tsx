import { useRef } from 'react';

// Component
import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalFooter,
  Input,
  Button,
} from '@chakra-ui/react';

// redux
import { useDispatch } from 'react-redux';
import { AppDispatch } from 'redux/store';
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
          GitHub personal token not found.
        </ModalHeader>
        <ModalBody>
          <p>Please enter token.</p>
          <p>tokenを入力してください。</p>
          <Input
            type="text"
            ref={inputElement}
            autoComplete="off"
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
