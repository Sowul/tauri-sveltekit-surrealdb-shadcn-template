create base:cpt set
    name = 'cpt',
    created_at = time::now()
;

update base:cpt set last_used = time::now()
;

create base:twic set
    name = 'twic',
    created_at = time::now()
;

update base:twic set last_used = time::now()
;