pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub position: usize,
}

impl BytePacketBuffer {
    #[warn(dead_code)] 
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0;512],
            position: 0
        }
    }

    // Current position in the buffer
    pub fn pos(&self) -> usize {
        self.position
    }

    // Step specific number of steps forward in te buffer
    pub fn step(&mut self, steps: usize) -> Result<(), ()> {
        let future_position = self.pos() + steps;

        // Check if steps are not going to 
        // be bigger than the buffer size
        if future_position > self.buf.len() {
            return Err(());
        }

        self.position = future_position;
        Ok(())
    }

    // Change buffer position
    pub fn seek(&mut self, pos: usize) -> Result<(), ()> {
        // Check if steps are not going to 
        // be bigger than the buffer size
        if pos > self.buf.len() {
            return Err(());
        }

        self.position = pos;
        Ok(())
    }

    // Retun value of the current buffer position and
    // jump one step forward
    pub fn read(&mut self) -> Result<u8, ()> {
        // Check if steps are not going to be bigger
        // than the buffer size
        if self.pos() >= self.buf.len() {
            return Err(());
        }

        let result:u8 = self.buf[self.pos()];
        // Jump one step forward
        self.step(1).ok().unwrap_or(());
        Ok(result)
    }

    // Return the current buffer data in position
    pub fn get(&mut self, pos:usize) -> Result <u8, ()> {
        if pos >= self.buf.len() {
            return Err(());
        }
        Ok(self.buf[pos])
    }

    // Retunr range of selected bytes
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], ()> {
        // Check if len are not going to
        // be bigger than buf size
        if start + len > self.buf.len() {
            return Err(());
        }

        Ok(&self.buf[start..start + len])
    }

    // Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16, ()> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    // Read four bytes, stepping four steps forward
    pub fn read_u32(&mut self) -> Result<u32, ()> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(res)
    }

    /// Read a qname
    ///
    /// The tricky part: Reading domain names, taking labels into consideration.
    /// Will take something like [3]www[6]google[3]com[0] and append
    /// www.google.com to outstr.
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), ()> {
        // Since we might encounter jumps, we'll keep track of our position
        // locally as opposed to using the position within the struct. This
        // allows us to move the shared position to a point past our current
        // qname, while keeping track of our progress on the current qname
        // using this variable.
        let mut pos = self.pos();

        // track whether or not we've jumped
        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        // Our delimiter which we append for each label. Since we don't want a
        // dot at the beginning of the domain name we'll leave it empty for now
        // and set it to "." at the end of the first iteration.
        let mut delim = "";
        loop {
            // Dns Packets are untrusted data, so we need to be paranoid. Someone
            // can craft a packet with a cycle in the jump instructions. This guards
            // against such packets.
            if jumps_performed > max_jumps {
                return Err(());
            }

            // At this point, we're always at the beginning of a label. Recall
            // that labels start with a length byte.
            let len = self.get(pos)?;

            // If len has the two most significant bit are set, it represents a
            // jump to some other offset in the packet:
            if (len & 0xC0) == 0xC0 {
                // Update the buffer position to a point past the current
                // label. We don't need to touch it any further.
                if !jumped {
                    self.seek(pos + 2)?;
                }

                // Read another byte, calculate offset and perform the jump by
                // updating our local position variable
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                // Indicate that a jump was performed.
                jumped = true;
                jumps_performed += 1;

                continue;
            }
            // The base scenario, where we're reading a single label and
            // appending it to the output:
            else {
                // Move a single byte forward to move past the length byte.
                pos += 1;

                // Domain names are terminated by an empty label of length 0,
                // so if the length is zero we're done.
                if len == 0 {
                    break;
                }

                // Append the delimiter to our output buffer first.
                outstr.push_str(delim);

                // Extract the actual ASCII bytes for this label and append them
                // to the output buffer.
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";

                // Move forward the full length of the label.
                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod byte_packet_buffer_test {
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

    #[test]
    fn func_seek_should_change_buf_pos() {
        let mut byte_packet = BytePacketBuffer::new();
        let result = byte_packet.seek(8);

        assert_eq!(result, Ok(()));
        assert_eq!(byte_packet.pos(), 8);
    }

    #[test]
    fn func_seek_should_not_change_pos_if_bugger_than_buff_length() {
        let mut byte_packet = BytePacketBuffer::new();
        let buf_length = byte_packet.buf.len();

        let result = byte_packet.seek(buf_length + 1);

        assert_eq!(result, Err(()));
        assert_eq!(byte_packet.pos(), 0);
    }

    #[test]
    fn func_read_should_return_the_buf_data_in_position_index_and_jump_one_step_forward() {
        let mut byte_packet = BytePacketBuffer::new();
        let old_position = byte_packet.pos();
        let result = byte_packet.read();

        assert_eq!(result, Ok(0));
        assert_eq!(byte_packet.pos(), old_position + 1);
    }

    #[test]
    fn func_read_should_err_if_cant_got_one_step_forward() {
        let mut byte_packet = BytePacketBuffer::new();
        byte_packet.seek(512).ok().unwrap_or(());
        let result = byte_packet.read();
        let old_position = byte_packet.pos();

        assert_eq!(result, Err(()));
        assert_eq!(byte_packet.pos(), old_position);
    }

    #[test]
    fn func_get_return_current_data_in_buf_pos() {
        let mut byte_packet = BytePacketBuffer::new();
        let result = byte_packet.get(0);

        assert_eq!(result, Ok(byte_packet.buf[0]));
    }

    #[test]
    fn func_get_range_should_return_range_of_bytes() {
        let mut byte_packet = BytePacketBuffer::new();
        let result = byte_packet.get_range(0, 100);
        let fake_array:[u8; 100] = [0; 100];

        assert_eq!(result.expect("returns a array of selected range").len(), 100);
        assert_eq!(result.expect("returns a range of selected range"), fake_array);
    }
    
    #[test]
    fn func_get_range_should_return_err_if_range_is_bigger_than_buf_size() {
        let mut byte_packet = BytePacketBuffer::new();
        let result = byte_packet.get_range(0, byte_packet.buf.len() + 1);

        assert_eq!(result, Err(()));
    }

    
}
