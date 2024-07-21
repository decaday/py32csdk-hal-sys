import re

def generate_wrappers(mcu, header_file):
    with open(header_file, 'r') as file:
        content = file.read()

    pattern = re.compile(r'#define\s+(__HAL\w+)\((.*?)\)')
    matches = pattern.findall(content)
    # print(matches)

    header_output = []
    source_output = []

    for match in matches:
        macro_name = match[0]
        params = match[1]
        # body = match[2]
        # args = match[3]

        if mcu == "py32f030":
            # it is SDK's fault
            if "HAL_RCC_FLASH_FORCE_RESET" in macro_name:
                continue
            if "HAL_RCC_FLASH_RELEASE_RESET" in macro_name:
                continue

        if params == "":

            wrapper_name = macro_name.replace('__HAL', 'HAL')
            header_output.append(f'void {wrapper_name}();')

            source_output.append(f'void {wrapper_name}() {{')
            source_output.append(f'    {macro_name}();')
            source_output.append('}\n')

        else:
            pass

    return '\n'.join(header_output), '\n'.join(source_output)

header_file = '../PY32F0_Drivers/PY32F0xx_HAL_Driver/Inc/py32f0xx_hal_rcc.h'

header_output, source_output = generate_wrappers("py32f030", header_file)



print("Header File Output:")
print(header_output)
print("\nSource File Output:")
print(source_output)
