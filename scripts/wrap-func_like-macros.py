import re

def generate_wrappers(header_file):
    with open(header_file, 'r') as file:
        content = file.read()

    # 正则表达式匹配以__HAL开头的内联函数宏
    pattern = re.compile(r'#define\s+(__HAL\w+)\s*\((.*?)\)\s*(.*?)\s*\((.*?)\)')
    matches = pattern.findall(content)
    # print(matches)

    header_output = []
    source_output = []

    for match in matches:
        macro_name = match[0]
        params = match[1]
        body = match[2]
        args = match[3]

        # 生成包装函数
        wrapper_name = macro_name.replace('__HAL', 'HAL')
        header_output.append(f'inline void {wrapper_name}();')

        source_output.append(f'void {wrapper_name}() {{')
        source_output.append(f'    {macro_name}();')
        source_output.append('}\n')

    return '\n'.join(header_output), '\n'.join(source_output)

header_file = '../PY32F0_Drivers/PY32F0xx_HAL_Driver/Inc/py32f0xx_hal_rcc.h'
header_output, source_output = generate_wrappers(header_file)

print("Header File Output:")
print(header_output)
print("\nSource File Output:")
print(source_output)
