'''
# QUIC packet

Packet definitions of QUIC for Scapy

*This started development in context of a master's thesis at Hasselt University but has since moved here*

Sources:
- E. Gagliardi and O. Levillain, “Analysis of quic session establishment and its implementations,” in IFIP International Conference on Information Security Theory and Practice. Springer, 2019, pp. 169–184.
- https://scapy.readthedocs.io/en/latest/build_dissect.html#
- https://www.rfc-editor.org/rfc/rfc9000.html#name-1-rtt-packet
'''

from typing import List
from scapy.fields import (BitEnumField, BitField, StrField)
from scapy.packet import Packet


class QUIC_1RTT(Packet):
    '''
    # 1-RTT QUIC packets for Scapy

    Attributes
    ----------
    name:
        Constant: "1-RTT QUIC"

    fields_desc:
        List of fields in the header

    Methods
    -------
    protect(self, pp_key: bytes, iv: bytes, hp_key: bytes):
        Protect (encrypt) the packet

    decrypt(self, pp_key: bytes, iv: bytes, hp_key: bytes):
        Decrypt the packet
    '''
    name: str = "1-RTT QUIC"
    fields_desc: List = [
        # Flags
        BitEnumField("header_form", 0, 1,
                     {0: "0 (short)",
                      1: "1 (long)"}),
        BitEnumField("fixed_bit", 1, 1,
                     {0: "0 (illegal)",
                      1: "1"}),
        BitField("spin_bit", 1, 1,),
        BitEnumField("reserved", 0, 2,
                     {0: "0",
                      1: "1 (illegal)",
                      2: "2 (illegal)",
                      3: "3 (illegal)"}),
        BitField("key_phase", 0, 1),
        BitField("packet_nr_len", 0, 2),
        # Destination Connection ID
        StrField("dcid", b'\x00'),
        # Packet number
        StrField("packet_nr", 0)
    ]

    def protect(self, pp_key: bytes, iv: bytes, hp_key: bytes):
        '''
        # Protect (encrypt)

        Encrypt an 1-RTT QUIC packet

        Parameters
        ----------
        `pp_key`:
            Packet protection key
        `iv`:     
            Initialization vector
        `hp_key`:  
            Header protection key

        Returns
        -------
        Nothing
        '''
        # Todo: Protect 1-rtt
        raise NotImplementedError

    def decrypt(self, pp_key: bytes, iv: bytes, hp_key: bytes):
        '''
        # Decrypt

        Decrypt an 1-RTT QUIC packet

        Parameters
        ----------
        `pp_key`: 
            Packet protection key
        `iv`:     
            Initialization vector
        `hp_key`:
            Header protection key

        Returns
        -------
        Nothing
        '''
        # Todo: Decrypt 1-rtt
        raise NotImplementedError

# Todo: Define packet types other than 1-RTT
# class QUIC_version_negotiation(Packet):
# class QUIC_initial(Packet):
# class QUIC_0RTT(Packet):
# class QUIC_handshake(Packet):
# class QUIC_retry(Packet):
