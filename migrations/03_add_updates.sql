UPDATE project
SET status_id = 4
WHERE title = 'portfolio-ssr';


INSERT INTO project (status_id, title, description, stacks, url_to_project)
VALUES
    (
        2,
        'Reflex',
        'Reflex est un projet de lunettes connectées permettant d''''enregistrer les dernières secondes d''''une scène grâce à un buffer circulaire et un bouton Replay. L''''objectif est de proposer une solution autonome, robuste et respectueuse de la vie privée.',
        (
            SELECT json_group_array(id)
            FROM technology
            WHERE name IN ('Rust', 'Leptos', 'Axum', 'Tokio', 'Tauri')
        ),
        NULL
    ),
    (
        2,
        'SocialPac',
        'SocialPac est une plateforme web full-stack permettant de visualiser les relations entre individus au travers d''un graphe interactif.',
        (
            SELECT json_group_array(id)
            FROM technology
            WHERE name IN ('Leptos', 'Rust', 'Axum', 'PostgreSQL', 'Tokio', 'Refinery', 'Sqlx')
        ),
        'https://github.com/The-Pac/SocialPac'
    );