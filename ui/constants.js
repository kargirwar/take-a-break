class Constants {
    static get PAGE_CHANGE() {
        return 'page-change';
    }

    static get PAGE_RULE_EDITOR() {
        return 'page-rule-editor';
    }

    static get PAGE_INDEX() {
        return 'page-index';
    }

    static get PAGE_RULES() {
        return 'page-rules';
    }

    //js events
    static get EVENT_DOM_LOADED() {
        return 'event-dom-loaded';
    }

    static get EVENT_RULES_UPDATED() {
        return 'event-rules-updated';
    }

    static get EVENT_NEXT_ALARM() {
        return 'event-next-alarm';
    }

    //for rust
    static get CMD_UPDATE_RULES() {
        return 'update-rules';
    }

    static get CMD_STARTUP() {
        return 'startup';
    }

    //from rust
    static get EVENT_RULES_APPLIED() {
        return 'event-rules-applied';
    }
}

export { Constants }
