import gdb

VARIANT_NAMES = ['Empty', 'Bytes', 'ArcStr',
                 'ArcString', 'StaticStr', 'Inline']


class FastStrPrettyPrinter:
    def __init__(self, valobj: gdb.Value):
        repr_obj = valobj['__0']
        inner = repr_obj[repr_obj.type.fields()[0]]
        fields = inner.type.fields()
        discr = int(inner[fields[0]]) + 1
        field_name = fields[discr].name
        assert field_name == VARIANT_NAMES[discr-1]
        variant = inner[fields[discr]]

        self._variant_name = field_name
        self._display_string = ''
        self._is_error = False

        if discr == 1:
            pass
        elif discr == 2:
            self._display_string = self._extract_bytes(variant)
        elif discr == 3:
            self._display_string = self._extract_arc_str(variant)
        elif discr == 4:
            self._display_string = self._extract_arc_string(variant)
        elif discr == 5:
            self._display_string = self._extract_static_str(variant)
        elif discr == 6:
            self._display_string = self._extract_inline(variant)
        else:
            self._display_string = '<Invalid FastStr>'
            self._is_error = True

    def _extract_bytes(self, variant: gdb.Value) -> str:
        try:
            bytes_obj = variant['__0']
            length = int(bytes_obj['len'])
            ptr = bytes_obj['ptr']
            return ptr.string('utf-8', length=length)
        except Exception as e:
            self._is_error = True
            return f'<Error reading Bytes: {e}>'

    def _extract_arc_str(self, variant: gdb.Value) -> str:
        try:
            arc_obj = variant['__0']
            non_null_ptr = arc_obj['ptr']
            arc_inner_ptr = non_null_ptr['pointer']
            data_ptr = arc_inner_ptr['data_ptr']
            length = int(arc_inner_ptr['length'])
            arc_inner = data_ptr.dereference()
            str_data = arc_inner['data']
            data = gdb.selected_inferior().read_memory(str_data.address, length)
            return data.tobytes().decode('utf-8')
        except Exception as e:
            self._is_error = True
            return f'<Error reading ArcStr: {e}>'

    def _extract_arc_string(self, variant: gdb.Value) -> str:
        try:
            arc_obj = variant['__0']
            non_null_ptr = arc_obj['ptr']
            arc_inner_ptr = non_null_ptr['pointer']
            arc_inner = arc_inner_ptr.dereference()
            string_obj = arc_inner['data']
            vec_obj = string_obj['vec']
            length = int(vec_obj['len'])
            buf = vec_obj['buf']
            raw_vec_inner = buf['inner']
            unique_ptr = raw_vec_inner['ptr']
            non_null_u8_ptr = unique_ptr['pointer']
            ptr = non_null_u8_ptr['pointer']
            return ptr.string('utf-8', length=length)
        except Exception as e:
            self._is_error = True
            return f'<Error reading ArcString: {e}>'

    def _extract_static_str(self, variant: gdb.Value) -> str:
        try:
            str_ref = variant['__0']
            length = int(str_ref['length'])
            data_ptr = str_ref['data_ptr']
            return data_ptr.string('utf-8', length=length)
        except Exception as e:
            self._is_error = True
            return f'<Error reading StaticStr: {e}>'

    def _extract_inline(self, variant: gdb.Value) -> str:
        try:
            length = int(variant['len'])
            buf = variant['buf']
            data = bytes([int(buf[i]) for i in range(length)])
            return data.decode('utf-8')
        except Exception as e:
            self._is_error = True
            return f'<Error reading Inline: {e}>'

    def to_string(self) -> str:
        return self._display_string if self._is_error else f'FastStr::{self._variant_name}("{self._display_string}")'


def faststr_pretty_printer(valobj: gdb.Value):
    type_name = str(valobj.type.strip_typedefs())
    if type_name == 'faststr::FastStr':
        return FastStrPrettyPrinter(valobj)
    return None


gdb.pretty_printers.append(faststr_pretty_printer)
