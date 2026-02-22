import {useState} from 'react';
import * as Select from '@radix-ui/react-select';
import '../../../shared/select.css';

export default function SelectPage() {
    const [value, setValue] = useState('');
    const [disabled, setDisabled] = useState(false);

    return (
        <>
            <Select.Root value={value} onValueChange={setValue} disabled={disabled}>
                <Select.Trigger className="select-trigger" data-testid="select-trigger">
                    <Select.Value placeholder="Select a fruit..." />
                    <Select.Icon className="select-icon">▼</Select.Icon>
                </Select.Trigger>
                <Select.Portal>
                    <Select.Content className="select-content" position="popper" sideOffset={4}>
                        <Select.ScrollUpButton className="select-scroll-button">▲</Select.ScrollUpButton>
                        <Select.Viewport className="select-viewport">
                            <Select.Group>
                                <Select.Label className="select-label">Fruits</Select.Label>
                                <Select.Item className="select-item" value="apple">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Apple</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="banana">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Banana</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="cherry" disabled>
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Cherry</Select.ItemText>
                                </Select.Item>
                            </Select.Group>

                            <Select.Separator className="select-separator" />

                            <Select.Group>
                                <Select.Label className="select-label">Vegetables</Select.Label>
                                <Select.Item className="select-item" value="carrot">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Carrot</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="potato">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Potato</Select.ItemText>
                                </Select.Item>
                            </Select.Group>
                        </Select.Viewport>
                        <Select.ScrollDownButton className="select-scroll-button">▼</Select.ScrollDownButton>
                    </Select.Content>
                </Select.Portal>
            </Select.Root>

            <br />
            <br />

            <label>
                <input type="checkbox" checked={disabled} onChange={(e) => setDisabled(e.target.checked)} />{' '}
                disabled
            </label>

            <br />
            <br />

            <span data-testid="select-value">{value || '(none)'}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>
            <input data-testid="outside-input" placeholder="name" />
        </>
    );
}
