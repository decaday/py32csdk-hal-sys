import re
import os

def extract_vectors(startup_file):
    with open(startup_file, 'r') as file:
        content = file.read()

    # Extract the vector table
    vectors = re.findall(r'; External Interrupts\s+(.+?)__Vectors_End', content, re.DOTALL)
    if not vectors:
        raise ValueError("No vector table found between __Vectors and __Vectors_End")
    # print(vectors[0])
    return vectors[0].strip().split('\n')

def generate_interrupt_rs(vectors, output_dir):
    handlers = []
    for line in vectors:
        match = re.match(r'\s*DCD\s+(\w+)', line)
        if match:
            handler = match.group(1)
            if handler == '0':
                handlers.append('Vector { reserved: 0 }')
            else:
                handlers.append(f'Vector {{ handler: {handler} }}')

    interrupt_rs_content = '''
pub union Vector {{
    handler: unsafe extern "C" fn(),
    reserved: usize,
}}

extern "C" {{
    {}
}}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; {}] = [
    {},
];
'''.format("".join([f"fn {handler}();\n    " for handler in set(re.findall(r'handler: (\w+)', "\n".join(handlers))) if handler != '0']),\
    len(handlers),\
    ",\n    ".join(handlers))

    with open(os.path.join(output_dir, 'interrupt.rs'), 'w') as file:
        file.write(interrupt_rs_content)

def generate_device_x(vectors, output_dir):
    handlers = set(re.findall(r'\s*DCD\s+(\w+)', "\n".join(vectors)))
    handlers.discard('0')

    device_x_content = '/* device.x */\n' + '\n'.join([f'PROVIDE({handler} = DefaultHandler);' for handler in handlers])

    with open(os.path.join(output_dir, 'device.x'), 'w') as file:
        file.write(device_x_content)

def main():
    startup_file = '../PY32F0_Drivers/CMSIS/Device/PY32F0xx/Source/arm/startup_py32f030x8.s'
    output_dir = '../output'

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    vectors = extract_vectors(startup_file)
    # print(vectors)
    generate_interrupt_rs(vectors, output_dir)
    generate_device_x(vectors, output_dir)

if __name__ == '__main__':
    main()
