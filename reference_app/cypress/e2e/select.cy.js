describe('Select', () => {
    // ── Helpers ──────────────────────────────────────────────

    function shouldBeOpen() {
        cy.findByRole('listbox').should('exist');
    }

    function shouldBeClosed() {
        // Select content may stay in DOM; check trigger data-state instead
        getTrigger().should('have.attr', 'data-state', 'closed');
    }

    // Use CSS selector since trigger gets aria-hidden when select is open (modal)
    function getTrigger() {
        return cy.get('.select-trigger');
    }

    function openSelect() {
        getTrigger().click();
        shouldBeOpen();
    }

    beforeEach(() => {
        cy.visit('/select');
    });

    // ── 1. Accessibility Semantics ──────────────────────────

    describe('accessibility', () => {
        it('Trigger has role="combobox"', () => {
            getTrigger().should('have.attr', 'role', 'combobox');
        });

        it('Trigger has aria-expanded', () => {
            getTrigger().should('have.attr', 'aria-expanded', 'false');
            openSelect();
            getTrigger().should('have.attr', 'aria-expanded', 'true');
        });

        it('Content has role="listbox"', () => {
            openSelect();
            cy.findByRole('listbox').should('exist');
        });

        it('Items have role="option"', () => {
            openSelect();
            cy.findAllByRole('option').should('have.length.at.least', 1);
        });

        it('Groups have role="group"', () => {
            openSelect();
            cy.findAllByRole('group').should('have.length', 2);
        });

        it('Groups are labelled by their Label', () => {
            openSelect();
            cy.findAllByRole('group')
                .first()
                .should('have.attr', 'aria-labelledby')
                .then((labelId) => {
                    cy.get(`#${labelId}`).should('have.text', 'Fruits');
                });
        });

        it('Trigger has aria-autocomplete="none"', () => {
            getTrigger().should('have.attr', 'aria-autocomplete', 'none');
        });
    });

    // ── 2. Data Attributes ──────────────────────────────────

    describe('data attributes', () => {
        it('Trigger data-state closed→open→closed', () => {
            getTrigger().should('have.attr', 'data-state', 'closed');
            openSelect();
            getTrigger().should('have.attr', 'data-state', 'open');
            cy.realPress('Escape');
            shouldBeClosed();
            getTrigger().should('have.attr', 'data-state', 'closed');
        });

        it('Trigger has data-placeholder when no value selected', () => {
            getTrigger().should('have.attr', 'data-placeholder');
        });

        it('Trigger loses data-placeholder after selection', () => {
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).click();
            shouldBeClosed();
            getTrigger().should('not.have.attr', 'data-placeholder');
        });

        it('Content has data-state="open"', () => {
            openSelect();
            cy.findByRole('listbox').should('have.attr', 'data-state', 'open');
        });

        it('Content has data-side attribute (popper mode)', () => {
            openSelect();
            cy.findByRole('listbox').should('have.attr', 'data-side');
        });

        it('Content has data-align attribute (popper mode)', () => {
            openSelect();
            cy.findByRole('listbox').should('have.attr', 'data-align');
        });

        it('Item data-state reflects checked/unchecked', () => {
            openSelect();
            // No item selected initially
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-state', 'unchecked');
            cy.findByRole('option', {name: 'Apple'}).click();
            shouldBeClosed();
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-state', 'checked');
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-state', 'unchecked');
        });

        it('Disabled item has data-disabled', () => {
            openSelect();
            cy.findByRole('option', {name: 'Cherry'}).should('have.attr', 'data-disabled');
        });

        it('Item has data-highlighted when focused', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.findAllByRole('option').not('[data-disabled]').first().should('have.attr', 'data-highlighted');
        });
    });

    // ── 3. Keyboard Navigation ──────────────────────────────

    describe('keyboard navigation', () => {
        it('Space opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('Space');
            shouldBeOpen();
        });

        it('Enter opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('Enter');
            shouldBeOpen();
        });

        it('ArrowDown opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
        });

        it('ArrowUp opens select from trigger', () => {
            getTrigger().focus();
            cy.realPress('ArrowUp');
            shouldBeOpen();
        });

        it('ArrowDown navigates to next option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            // Should move to next non-disabled option
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-highlighted');
        });

        it('ArrowUp navigates to previous option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowUp');
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-highlighted');
        });

        it('Enter selects highlighted item', () => {
            // Open via click, then use keyboard to navigate and select
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown'); // Banana
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-highlighted');
            cy.realPress('Enter');
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', 'banana');
        });

        it('Space selects and closes', () => {
            getTrigger().focus();
            cy.realPress('Space');
            shouldBeOpen();
            // First option highlighted
            cy.realPress('Space');
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', 'apple');
        });

        it('Escape closes without selecting', () => {
            openSelect();
            cy.realPress('Escape');
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', '(none)');
        });

        it('Home moves to first option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('ArrowDown');
            cy.realPress('ArrowDown');
            cy.realPress('Home');
            cy.findByRole('option', {name: 'Apple'}).should('have.attr', 'data-highlighted');
        });

        it('End moves to last option', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            cy.realPress('End');
            cy.findByRole('option', {name: 'Potato'}).should('have.attr', 'data-highlighted');
        });

        it('ArrowDown skips disabled options', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            // Starts on Apple
            cy.realPress('ArrowDown');
            // Banana
            cy.realPress('ArrowDown');
            // Should skip Cherry (disabled) and go to Carrot
            cy.findByRole('option', {name: 'Carrot'}).should('have.attr', 'data-highlighted');
        });
    });

    // ── 4. Pointer Interaction ──────────────────────────────

    describe('pointer interaction', () => {
        it('click trigger opens select', () => {
            getTrigger().click();
            shouldBeOpen();
        });

        it('click option selects and closes', () => {
            openSelect();
            cy.findByRole('option', {name: 'Banana'}).click();
            shouldBeClosed();
            cy.findByTestId('select-value').should('have.text', 'banana');
        });

        it('click outside closes select', () => {
            openSelect();
            cy.get('body').realClick({position: {x: 1, y: 1}});
            shouldBeClosed();
        });
    });

    // ── 5. Focus Management ─────────────────────────────────

    describe('focus management', () => {
        it('focus returns to trigger on close', () => {
            openSelect();
            cy.realPress('Escape');
            shouldBeClosed();
            getTrigger().should('be.focused');
        });

        it('selected item is highlighted when reopening', () => {
            // Select Banana
            openSelect();
            cy.findByRole('option', {name: 'Banana'}).click();
            shouldBeClosed();
            // Reopen — Banana should be highlighted
            openSelect();
            cy.findByRole('option', {name: 'Banana'}).should('have.attr', 'data-highlighted');
        });
    });

    // ── 6. Value Display ────────────────────────────────────

    describe('value display', () => {
        it('shows placeholder initially', () => {
            getTrigger().should('contain.text', 'Select a fruit...');
        });

        it('shows selected item text in trigger', () => {
            openSelect();
            cy.findByRole('option', {name: 'Carrot'}).click();
            shouldBeClosed();
            getTrigger().should('contain.text', 'Carrot');
        });

        it('readout updates on selection', () => {
            cy.findByTestId('select-value').should('have.text', '(none)');
            openSelect();
            cy.findByRole('option', {name: 'Apple'}).click();
            cy.findByTestId('select-value').should('have.text', 'apple');
        });
    });

    // ── 7. Disabled State ───────────────────────────────────

    describe('disabled state', () => {
        it('disabled trigger cannot be opened', () => {
            cy.findByLabelText('disabled').click();
            getTrigger().should('have.attr', 'data-disabled');
            getTrigger().click({force: true});
            shouldBeClosed();
        });

        it('disabled item has data-disabled and cannot be selected', () => {
            openSelect();
            cy.findByRole('option', {name: 'Cherry'}).should('have.attr', 'data-disabled');
            cy.findByRole('option', {name: 'Cherry'}).click();
            // Select should stay open — disabled item doesn't select
            shouldBeOpen();
        });
    });

    // ── 8. Groups ───────────────────────────────────────────

    describe('groups', () => {
        it('navigation continues across groups', () => {
            getTrigger().focus();
            cy.realPress('ArrowDown');
            shouldBeOpen();
            // Navigate through: Apple → Banana → (skip Cherry) → Carrot → Potato
            cy.realPress('ArrowDown'); // Banana
            cy.realPress('ArrowDown'); // Carrot (skips disabled Cherry, crosses group boundary)
            cy.findByRole('option', {name: 'Carrot'}).should('have.attr', 'data-highlighted');
            cy.realPress('ArrowDown'); // Potato
            cy.findByRole('option', {name: 'Potato'}).should('have.attr', 'data-highlighted');
        });
    });
});
