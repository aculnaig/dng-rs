use nom::{bytes::complete::take, number::complete::{be_u16, be_u32}, IResult};

// TODO: parse IFDs after the first one
// TODO: change IResult<I, O> to Result<O, E> where O: Output, E: Error
pub struct TiffFile {
    pub header: TiffHeader,
}

impl TiffFile {
    pub fn new(byte_order: TiffByteOrder, first_ifd_offset: u32) -> TiffFile {
        TiffFile {
            header: TiffHeader::new(byte_order, first_ifd_offset),
        }
    }
}

pub struct TiffHeader {
    pub byte_order: TiffByteOrder,
    pub version: u16,
    pub first_ifd_offset: u32,
}

impl TiffHeader {
    pub fn new(byte_order: TiffByteOrder, first_ifd_offset: u32) -> TiffHeader {
        TiffHeader {
            byte_order,
            version: 0x2A,
            first_ifd_offset,
        }
    }
}

pub struct TiffIfd {
    pub num_entries: u16,
    pub directory_entries: Vec<TiffIfdEntry>,
    pub next_ifd_offset: u32,
}

pub struct TiffIfdEntry {
    pub tag: TiffTag,
    pub field_type: TiffDataType,
    pub count: u32,
    pub value_offset: u32,
}

#[derive(Debug)]
pub enum TiffTag {
    ImageWidth,
    ImageLength,
    BitsPerSample,
    Compression,
    PhotometricInterpretation,
    StripOffsets,
    RowsPerStrip,
    StripByteCounts,
    XResolution,
    YResolution,
    ResolutionUnit,
    ColorMap,
    ExtraSamples,
    SampleFormat,
    TransferFunction,
    Software,
    DateTime,
    Artist,
    HostComputer,
    Predictor,
    WhitePoint,
    PrimaryChromaticities,
    ColorMapBitDepth,
    TileWidth,
    TileLength,
    TileOffsets,
    TileByteCounts,
    SubIFDs,
    InkSet,
    InkNames,
    NumberOfInks,
    DotRange,
    TargetPrinter,
    ExtraSamples2,
    SampleFormat2,
    TransferFunction2,
    JPEGTables,
    OFBData,
    NewSubFileType,
    SubIFDOffset,
    ImageLayer,
    End,
}

impl TiffTag {
    pub fn from_u16(value: u16) -> TiffTag {
        match value {
            256 => TiffTag::ImageWidth,
            257 => TiffTag::ImageLength,
            258 => TiffTag::BitsPerSample,
            259 => TiffTag::Compression,
            262 => TiffTag::PhotometricInterpretation,
            273 => TiffTag::StripOffsets,
            278 => TiffTag::RowsPerStrip,
            279 => TiffTag::StripByteCounts,
            282 => TiffTag::XResolution,
            283 => TiffTag::YResolution,
            296 => TiffTag::ResolutionUnit,
            320 => TiffTag::ColorMap,
            338 => TiffTag::ExtraSamples,
            339 => TiffTag::SampleFormat,
            301 => TiffTag::TransferFunction,
            305 => TiffTag::Software,
            306 => TiffTag::DateTime,
            315 => TiffTag::Artist,
            316 => TiffTag::HostComputer,
            317 => TiffTag::Predictor,
            318 => TiffTag::WhitePoint,
            319 => TiffTag::PrimaryChromaticities,
            _ => panic!("Invalid TiffTag: {}", value),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            TiffTag::ImageWidth => 256,
            TiffTag::ImageLength => 257,
            TiffTag::BitsPerSample => 258,
            TiffTag::Compression => 259,
            TiffTag::PhotometricInterpretation => 262,
            TiffTag::StripOffsets => 273,
            TiffTag::RowsPerStrip => 278,
            TiffTag::StripByteCounts => 279,
            TiffTag::XResolution => 282,
            TiffTag::YResolution => 283,
            TiffTag::ResolutionUnit => 296,
            TiffTag::ColorMap => 320,
            TiffTag::ExtraSamples => 338,
            TiffTag::SampleFormat => 339,
            TiffTag::TransferFunction => 301,
            TiffTag::Software => 305,
            TiffTag::DateTime => 306,
            TiffTag::Artist => 315,
            TiffTag::HostComputer => 316,
            TiffTag::Predictor => 317,
            TiffTag::WhitePoint => 318,
            TiffTag::PrimaryChromaticities => 319,
            _ => panic!("Invalid TiffTag: {:?}", self),
        }
    }
}

pub enum TiffDataType {
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    SByte,
    Undefined,
    SShort,
    SLong,
    SRational,
    Float,
    Double,
}

impl TiffDataType {
    pub fn from_u16(value: u16) -> TiffDataType {
        match value {
            1 => TiffDataType::Byte,
            2 => TiffDataType::Ascii,
            3 => TiffDataType::Short,
            4 => TiffDataType::Long,
            5 => TiffDataType::Rational,
            6 => TiffDataType::SByte,
            7 => TiffDataType::Undefined,
            8 => TiffDataType::SShort,
            9 => TiffDataType::SLong,
            10 => TiffDataType::SRational,
            11 => TiffDataType::Float,
            12 => TiffDataType::Double,
            _ => panic!("Invalid data type"),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            TiffDataType::Byte => 1,
            TiffDataType::Ascii => 2,
            TiffDataType::Short => 3,
            TiffDataType::Long => 4,
            TiffDataType::Rational => 5,
            TiffDataType::SByte => 6,
            TiffDataType::Undefined => 7,
            TiffDataType::SShort => 8,
            TiffDataType::SLong => 9,
            TiffDataType::SRational => 10,
            TiffDataType::Float => 11,
            TiffDataType::Double => 12,
        }
    }
}

pub enum TiffByteOrder {
    LittleEndian,
    BigEndian,
}

impl TiffByteOrder {
    pub fn from_u16(value: u16) -> TiffByteOrder {
        match value {
            0x4949 => TiffByteOrder::LittleEndian,
            0x4D4D => TiffByteOrder::BigEndian,
            _ => panic!("Invalid byte order"),
        }
    }

    pub fn from_str(value: &str) -> TiffByteOrder {
        match value {
            "II" => TiffByteOrder::LittleEndian,
            "MM" => TiffByteOrder::BigEndian,
            _ => panic!("Invalid byte order"),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            TiffByteOrder::LittleEndian => 0x4949,
            TiffByteOrder::BigEndian => 0x4D4D,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TiffByteOrder::LittleEndian => "II",
            TiffByteOrder::BigEndian => "MM",
        }
    }
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], TiffHeader> {
    let (input, byte_order) = be_u16(input)?;
    let (input, _version) = be_u16(input)?;
    let (input, first_ifd_offset) = be_u32(input)?;

    let byte_order = TiffByteOrder::from_u16(byte_order);

    Ok((
        input,
        TiffHeader::new(byte_order, first_ifd_offset),
    ))
}

pub fn parse_directory_entry(input: &[u8]) -> IResult<&[u8], TiffIfdEntry> {
    let (input, tag) = be_u16(input)?;
    let (input, field_type) = be_u16(input)?;
    let (input, count) = be_u32(input)?;
    let (input, value_offset) = be_u32(input)?;

    Ok((
        input,
        TiffIfdEntry {
            tag: TiffTag::from_u16(tag),
            field_type: TiffDataType::from_u16(field_type),
            count,
            value_offset,
        },
    ))
}

pub fn parse_ifd(input: &[u8]) -> IResult<&[u8], TiffIfd> {
    let (input, num_entries) = be_u16(input)?;
    let (input, directory_entries) = take(num_entries * 12)(input)?;
    let (input, next_ifd_offset) = be_u32(input)?;

    let (_, directory_entries) = take(num_entries * 12)(directory_entries)?;

    let mut directory_entries = directory_entries;
    let mut entries = Vec::new();

    while !directory_entries.is_empty() {
        let (remaining, entry) = parse_directory_entry(directory_entries)?;
        directory_entries = remaining;
        entries.push(entry);
    }

    Ok((
        input,
        TiffIfd {
            num_entries,
            directory_entries: entries,
            next_ifd_offset,
        },
    ))
}

pub fn parse_tiff(input: &[u8]) -> IResult<&[u8], TiffFile> {
    let (input, header) = parse_header(input)?;

    Ok((input, TiffFile { header }))
}
