'''
# QUIC Generator

Generate QUIC packets using Scapy
'''

from math import ceil
from scapy.all import Packet
from quic_pkt import QUIC_1RTT

PRINT_PKT = True


def quic_1rtt():
    '''
    Build and print an 1-RTT QUIC packet
    '''
    # Setup
    spin_bit = 0x1
    key_phase = 0x1
    packet_number = 0xabc
    destination_connection_id = b'\xf3\xde\xc1\xbb\x98\xd9\x2f\x74\x02\xe8\x98\xce\xc8\x79\x54\x2c\xbd\x28\x23\x4c'
    payload = 'Hello World'.encode('utf-8')
    packet_number_length = ceil(packet_number.bit_length() / 8.0) - 1
    packet_number = packet_number.to_bytes(packet_number_length+1, 'big')

    # Create packet
    pkt: Packet = QUIC_1RTT(
        spin_bit=spin_bit,
        key_phase=key_phase,
        packet_nr_len=packet_number_length,
        dcid=destination_connection_id,
        packet_nr=packet_number
    )
    pkt.add_payload(payload)

    # Build packet
    pkt_bytes = pkt.build()

    # Print packet
    if PRINT_PKT:
        print('\n1-RTT QUIC Packet')
        print('-----------------')
        print(f'Packet len: {len(pkt_bytes)}')
        print(f'Packet: {pkt_bytes.hex()}\n')


if __name__ == '__main__':
    quic_1rtt()
