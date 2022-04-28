import { useEffect } from 'react';

// lib
import { invoke } from '@tauri-apps/api/tauri';

// component
import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  Text,
} from '@chakra-ui/react';

// Main
type PropTypes = {
  isOpen: boolean;
  onClose: () => void;
  msgTitle: string;
};

const ModalError = (props: PropTypes) => {
  const { isOpen, onClose, msgTitle } = props;

  useEffect(() => {
    const onAbnormalEnd = () => {
      setTimeout(() => invoke<void>('abnormal_end'), 3000);
    };

    onAbnormalEnd();
  });

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>{msgTitle}</ModalHeader>
        <ModalBody>
          <Text color="red" my={2}>
            Forced terminate app.
          </Text>
          <Text color="red" my={2}>
            強制終了します。
          </Text>
        </ModalBody>
      </ModalContent>
    </Modal>
  );
};

export default ModalError;
