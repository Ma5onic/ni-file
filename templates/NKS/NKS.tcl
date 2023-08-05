proc NKS {} {
	set magic [hex 4 "magic"]
	if {$magic != 0x1290A87F} {
		error "NKS magic must be 0x1290A87F, found $magic"
	}

	set zlibLength [uint32 "zlibLength"]

	set headerVersion [uint16 -hex "headerVersion"]

	if {$headerVersion == 0x0100} {
		sectionname "BPatchHeaderV2"
	}

	if {$headerVersion == 0x0110} {
		section "BPatchHeaderV42" {
			set headerMagic [hex 4 "headerMagic"]
			if {$headerMagic != 0x1A6337EA} {
				error "NKS headerMagic must be 0x1290A87F, found $headerMagic"
			}

			set patchType [uint16 "patchType"]

			uint8 "patchVersionMinorC"
			uint8 "patchVersionMinorB"
			uint8 "patchVersionMinorA"
			uint8 "patchVersionMajor"

			ascii 4 "appSignature"

			unixtime32 "creationDate"

			uint32 "?"

			uint16 "numZones"
			uint16 "numGroups"
			uint16 "numInstruments"
			bytes 16 "?"
			uint32 "icon"

			bytes 104 "strings"
			bytes 16 "checksum"

			uint32 "?"
			uint32 "?"
			uint32 "decompressedLength"

			bytes 32 "?"

			#set numStrings [uint32 "numStrings"]
		}
	}

	bytes $zlibLength "compressedSegment"

	section "footer" {
		hex 4 "footerMagic"
		uint8 ""
		uint8 ""
		uint16 ""
		set length [uint32 "length"]
		bytes $length "soundInfoItem"
	}
}

NKS
