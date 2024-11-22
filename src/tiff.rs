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