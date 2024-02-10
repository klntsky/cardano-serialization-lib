use crate::*;

impl cbor_event::se::Serialize for NativeScripts {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.len() as u64))?;
        for element in self.to_vec() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for NativeScripts {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut scripts = NativeScripts::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => scripts.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if raw.cbor_type()? == CBORType::Special {
                    assert_eq!(raw.special()?, CBORSpecial::Break);
                    break;
                }
                scripts.add_move(NativeScript::deserialize(raw)?);
            }
            Ok(())
        })()
            .map_err(|e| e.annotate("NativeScripts"))?;
        Ok(scripts)
    }
}