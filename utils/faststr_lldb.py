import lldb
import re

VARIANT_NAMES = ['Empty', 'Bytes', 'ArcStr',
                 'ArcString', 'StaticStr', 'Inline']


class FastStrSyntheticProvider:
    def __init__(self, valobj: lldb.SBValue, _internal_dict: dict):
        # IMPORTANT: use non-synthetic value instead of formatted value by Rust official lldb plugin
        self.valobj = valobj.GetNonSyntheticValue()
        self.update()

    def update(self):
        repr = self.valobj.GetChildAtIndex(0)
        variants = repr.GetChildMemberWithName('$variants$')
        discr = variants.GetChildAtIndex(0).GetChildMemberWithName(
            '$discr$').GetValueAsUnsigned()
        variant = variants.GetChildAtIndex(discr)
        variant_name = self._get_variant_name(variant)
        assert variant_name == VARIANT_NAMES[discr]

        self._variant_name = variant_name
        self._display_string = ''
        self._is_error = False

        if variant_name == 'Empty':
            self._display_string = ''
        elif variant_name == 'Bytes':
            self._display_string = self._extract_bytes(variant)
        elif variant_name == 'ArcStr':
            self._display_string = self._extract_arc_str(variant)
        elif variant_name == 'ArcString':
            self._display_string = self._extract_arc_string(variant)
        elif variant_name == 'StaticStr':
            self._display_string = self._extract_static_str(variant)
        elif variant_name == 'Inline':
            self._display_string = self._extract_inline(variant)
        else:
            self._display_string = '<Invalid FastStr>'
            self._is_error = True

    def _get_variant_name(self, variant: lldb.SBValue):
        full_name = variant.GetType().GetName()
        # faststr::Repr::faststr::Repr$Inner::Empty$Variant
        return re.match('^([a-zA-Z:]+)\\$Inner::([a-zA-Z]+)\\$Variant$', full_name).group(2)

    def _extract_bytes(self, variant: lldb.SBValue) -> str:
        bytes_obj = variant.GetChildMemberWithName(
            'value').GetChildMemberWithName('__0')
        ptr = bytes_obj.GetChildMemberWithName('ptr').GetValueAsUnsigned()
        length = bytes_obj.GetChildMemberWithName('len').GetValueAsUnsigned()
        error = lldb.SBError()
        data = variant.GetProcess().ReadMemory(ptr, length, error)
        if error.Success():
            return data.decode('utf-8')
        else:
            self._is_error = True
            return f'<Error reading Bytes: {error.GetCString()}>'

    def _extract_arc_str(self, variant: lldb.SBValue) -> str:
        # Arc<str>
        arc_str = variant.GetChildMemberWithName(
            'value').GetChildMemberWithName('__0')
        ptr = arc_str.GetChildMemberWithName('ptr')
        # ArcInner
        pointer = ptr.GetChildMemberWithName('pointer')
        # str
        data_ptr = pointer.GetChildMemberWithName('data_ptr')
        length = pointer.GetChildMemberWithName('length').GetValueAsUnsigned()
        data = data_ptr.GetChildMemberWithName('data')
        error = lldb.SBError()
        data = variant.GetProcess().ReadMemory(
            data.AddressOf().GetValueAsUnsigned(), length, error)
        if error.Success():
            return data.decode('utf-8')
        else:
            self._is_error = True
            return f'<Error reading ArcStr: {error.GetCString()}>'

    def _extract_arc_string(self, variant: lldb.SBValue) -> str:
        # Arc<String>
        arc_str = variant.GetChildMemberWithName(
            'value').GetChildMemberWithName('__0')
        ptr = arc_str.GetChildMemberWithName('ptr')
        # ArcInner
        arc_inner = ptr.GetChildMemberWithName('pointer')
        # String
        data = arc_inner.GetChildMemberWithName('data')
        # inner Vec<u8>
        vec = data.GetChildMemberWithName('vec')
        pointer = vec.GetChildMemberWithName('buf').GetChildMemberWithName('inner').GetChildMemberWithName(
            'ptr').GetChildMemberWithName('pointer').GetChildMemberWithName('pointer')
        length = vec.GetChildMemberWithName('len').GetValueAsUnsigned()
        error = lldb.SBError()
        data = variant.GetProcess().ReadMemory(
            pointer.GetValueAsUnsigned(), length, error)
        if error.Success():
            return data.decode('utf-8')
        else:
            self._is_error = True
            return f'<Error reading ArcString: {error.GetCString()}>'

    def _extract_static_str(self, variant: lldb.SBValue) -> str:
        # &'static str
        static_str = variant.GetChildMemberWithName(
            'value').GetChildMemberWithName('__0')
        data_ptr = static_str.GetChildMemberWithName('data_ptr')
        length = static_str.GetChildMemberWithName(
            'length').GetValueAsUnsigned()
        error = lldb.SBError()
        data = variant.GetProcess().ReadMemory(
            data_ptr.GetValueAsUnsigned(), length, error)
        if error.Success():
            return data.decode('utf-8')
        else:
            self._is_error = True
            return f'<Error reading StaticStr: {error.GetCString()}>'

    def _extract_inline(self, variant: lldb.SBValue) -> str:
        # inline
        inline = variant.GetChildMemberWithName('value')
        buf = inline.GetChildMemberWithName('buf')
        length = inline.GetChildMemberWithName('len').GetValueAsUnsigned()
        error = lldb.SBError()
        data = variant.GetProcess().ReadMemory(
            buf.AddressOf().GetValueAsUnsigned(), length, error)
        if error.Success():
            return data.decode('utf-8')
        else:
            self._is_error = True
            return f'<Error reading Inline: {error.GetCString()}>'

    def to_string(self) -> str:
        return self._display_string if self._is_error else f'FastStr::{self._variant_name}("{self._display_string}")'


def FastStrSummaryProvider(valobj: lldb.SBValue, internal_dict: dict) -> str:
    return FastStrSyntheticProvider(valobj, internal_dict).to_string()


def __lldb_init_module(debugger, internal_dict):
    debugger.HandleCommand(
        'type synthetic add -F faststr_lldb.VSCodeFastStrSyntheticProvider faststr::FastStr'
    )
    debugger.HandleCommand(
        'type summary add -F faststr_lldb.FastStrSummaryProvider faststr::FastStr'
    )
