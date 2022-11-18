pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub position: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0;512],
            position: 0
        }
    }

    pub fn pos(&self) -> usize {
        self.position
    }

    pub fn step(&mut self, steps: usize) -> Result<(), ()> {
        let future_position = self.pos() + steps;
        if future_position > self.buf.len() {
            return Err(());
        }
        self.position = future_position;
        Ok(())
    }
}

#[cfg(test)]
mod teste {
    use super::*;

    #[test]
    fn returns_empty_buffer() {
        let byte_packet = BytePacketBuffer::new();
        let byte_packet_emptry = BytePacketBuffer {
            buf: [0;512],
            position: 0
        };
        assert_eq!(byte_packet.buf, byte_packet_emptry.buf);
        assert_eq!(byte_packet.position, byte_packet_emptry.position);
    }

    #[test]
    fn func_pos_return_position_propertie() {
        let byte_packet = BytePacketBuffer::new();
        assert_eq!(byte_packet.pos(), byte_packet.position);
    }

    #[test]
    fn func_step_increment_one_in_buff_position() {
        let mut byte_packet = BytePacketBuffer::new();
        let return_of_step = byte_packet.step(2);
        
        assert_eq!(byte_packet.pos(), 2);
        assert_eq!(return_of_step, Ok(()));
    }

    #[test]
    fn func_step_return_err_if_steps_bigger_then_array(){
        let mut byte_packet = BytePacketBuffer::new();
        let return_of_step = byte_packet.step(byte_packet.buf.len() + 1);

        assert_eq!(return_of_step, Err(()));
        assert_eq!(byte_packet.pos(), 0);
    }
}
