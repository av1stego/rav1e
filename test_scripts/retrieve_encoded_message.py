import sys

END_LINE = "All data were trasmitted\n"
SKIPPING_IDENTIFIER = "[Skipping]"

def string_from_bits(bits):
    chars = []
    bytes_count = int(len(bits) / 8)
    for b in range(0, bytes_count):
        byte = bits[b*8:(b+1)*8]
        byte.reverse()
        chars.append(chr(int(''.join([str(bit) for bit in byte]), 2)))
    return ''.join(chars)

def retrieve_bits_from_lines(lines):
    bits = []

    for line in lines:
        if line == END_LINE:
            return bits 

        if SKIPPING_IDENTIFIER in line:
            continue

        bit = int(line.split(" ")[8])
        bits.append(bit)

    return bits

def main():
    lines = open(sys.argv[1]).readlines()
    bits = retrieve_bits_from_lines(lines)

    print(f"Injected bits: {len(bits)} ({len(bits)/8} bytes)")
    print(string_from_bits(bits))

if __name__ == "__main__":
    main()