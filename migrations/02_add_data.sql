INSERT INTO logo (id, path, name)
VALUES
-- MDI
(1, 'logo/mdi/diploma-icon.svg', 'Diplôme'),
(2, 'logo/mdi/study-icon.svg', 'Étude'),
(34, 'logo/mdi/location-icon.svg', 'Location'),
(39, 'logo/mdi/no-logo.svg', 'No logo'),

-- Front-End
(3, 'logo/programming_language/front_end/angular-icon.svg', 'Angular'),
(4, 'logo/programming_language/front_end/javascript-icon.svg', 'JavaScript'),
(5, 'logo/programming_language/front_end/reactjs-icon.svg', 'React.Js'),
(6, 'logo/programming_language/front_end/sass-icon.svg', 'Sass'),
(7, 'logo/programming_language/front_end/svelte-icon.svg', 'Svelte'),
(8, 'logo/programming_language/front_end/typescript-icon.svg', 'TypeScript'),
(9, 'logo/programming_language/front_end/vuejs-icon.svg', 'Vue.js'),
(10, 'logo/programming_language/front_end/css-icon.svg', 'CSS'),
(11, 'logo/programming_language/front_end/html5-icon.svg', 'HTML5'),
(40, 'logo/programming_language/front_end/qwik.svg', 'Qwik'),

-- Protocoles
(12, 'logo/programming_language/protocol/rest-api-icon.svg', 'REST API'),
(13, 'logo/programming_language/protocol/webrtc-icon.svg', 'WebRTC'),
(14, 'logo/programming_language/protocol/websocket-icon.svg', 'WebSocket'),

-- Bases de données
(15, 'logo/programming_language/database/mysql-icon.svg', 'MySQL'),
(16, 'logo/programming_language/database/postgresql-icon.svg', 'PostgreSQL'),
(17, 'logo/programming_language/database/sqlite-icon.svg', 'SQLite'),

-- Back-End
(18, 'logo/programming_language/back_end/java-icon.svg', 'Java'),
(19, 'logo/programming_language/back_end/laravel-icon.svg', 'Laravel'),
(20, 'logo/programming_language/back_end/leptos-icon.svg', 'Leptos'),
(21, 'logo/programming_language/back_end/lua-icon.svg', 'Lua'),
(22, 'logo/programming_language/back_end/nodejs-icon.svg', 'Node.js'),
(23, 'logo/programming_language/back_end/php-icon.svg', 'PHP'),
(24, 'logo/programming_language/back_end/rust-icon.svg', 'Rust'),
(25, 'logo/programming_language/back_end/spring-icon.svg', 'Spring'),
(26, 'logo/programming_language/back_end/tauri-icon.svg', 'Tauri'),
(29, 'logo/programming_language/back_end/actix-icon.svg', 'Actix'),
(30, 'logo/programming_language/back_end/axum-icon.svg', 'Axum'),

-- Plateformes
(27, 'logo/programming_language/platform/linux-icon.svg', 'Linux'),
(28, 'logo/programming_language/platform/windows-icon.svg', 'Windows'),

-- Librairies
(35, 'logo/programming_language/librairy/argon2.webp', 'Argon2'),
(36, 'logo/programming_language/librairy/barrel.webp', 'Barrel'),
(37, 'logo/programming_language/librairy/bevy.svg', 'Bevy'),
(41, 'logo/programming_language/librairy/refinery.svg', 'Refinery'),
(42, 'logo/programming_language/librairy/tokio.svg', 'Tokio'),
(43, 'logo/programming_language/librairy/tracing.webp', 'Tracing'),
(44, 'logo/programming_language/librairy/threejs.svg', 'Three.Js'),
(45, 'logo/programming_language/librairy/serde.webp', 'Serde'),
(50, 'logo/programming_language/librairy/vite-icon.svg', 'Vite'),

-- Tools
(38, 'logo/programming_language/tool/biomejs.svg', 'Biome.Js'),

-- DevOps & Infrastructure
(46, 'logo/programming_language/devops_and_infrastructure/github-icon.svg', 'Github'),
(47, 'logo/programming_language/devops_and_infrastructure/gitlab-icon.svg', 'Gitlab'),
(48, 'logo/programming_language/devops_and_infrastructure/jenkins-icon.svg', 'Jenkins'),
(49, 'logo/programming_language/devops_and_infrastructure/postman-icon.svg', 'Postman'),
(51, 'logo/programming_language/devops_and_infrastructure/docker-icon.svg', 'Docker'),

-- Entreprises
(31, 'logo/company/technopli.webp', 'Technopli'),
(32, 'logo/company/irouicome.webp', 'IrOuiCome'),
(33, 'logo/company/astree_software.webp', 'Astrée Software')
;

INSERT INTO technology_category (id, title, description)
VALUES (1, 'Front-End', 'Ce que l''utilisateur voit et avec quoi il interagit.'),
       (2, 'Protocole', 'Les règles qui permettent aux applications et sites de communiquer entre eux.'),
       (3, 'Base de donnée', 'Gestion et stockage des données d''une application ou d''un site web.'),
       (4, 'Back-End', 'Le back-end gère les données et le fonctionnement en arrière-plan.'),
       (5, 'Plateforme', 'Les plateformes sont les systèmes qui exécutent les applications.'),
       (6, 'Librairies', 'Ajoute des fonctionnalités prêtes à l''emploi pour simplifier le développement.'),
       (7, 'DevOps & Infrastructure',
        'Tout ce qui permet de déployer, faire tourner, automatiser et surveiller les applications en production.')
;

INSERT INTO technology (id, name, technology_category_id, logo_id)
VALUES
-- Front-End
(1, 'Angular', 1, 3),
(2, 'JavaScript', 1, 4),
(3, 'React', 1, 5),
(4, 'Sass', 1, 6),
(5, 'Svelte', 1, 7),
(6, 'TypeScript', 1, 8),
(7, 'Vue.js', 1, 9),
(8, 'CSS', 1, 10),
(9, 'HTML5', 1, 11),
(34, 'Qwik', 1, 40),

-- Protocoles
(10, 'REST API', 2, 12),
(11, 'WebRTC', 2, 13),
(12, 'WebSocket', 2, 14),

-- Bases de données
(13, 'MySQL', 3, 15),
(14, 'PostgreSQL', 3, 16),
(15, 'SQLite', 3, 17),

-- Back-End
(16, 'Java', 4, 18),
(17, 'Laravel', 4, 19),
(18, 'Leptos', 4, 20),
(19, 'Lua', 4, 21),
(20, 'Node.js', 4, 22),
(21, 'PHP', 4, 23),
(22, 'Rust', 4, 24),
(23, 'Spring', 4, 25),
(24, 'Tauri', 4, 26),
(27, 'Actix', 4, 29),
(28, 'Axum', 4, 30),

-- Plateformes
(25, 'Linux', 5, 27),
(26, 'Windows', 5, 28),

-- Librairies
(29, 'Argon2', 6, 35),
(30, 'Barrel', 6, 36),
(31, 'Bevy', 6, 37),
(35, 'Refinery', 6, 41),
(36, 'Sqlx', 6, 39),
(37, 'Tokio', 6, 42),
(38, 'Tracing', 6, 43),
(39, 'Three.Js', 6, 44),
(40, 'Serde', 6, 45),
(41, 'Vite', 6, 50),
(43, 'Thiserror', 6, 39),
(44, 'Eframe', 6, 39),

-- DevOps & Infrastructure
(45, 'Github', 7, 46),
(46, 'Gitlab', 7, 47),
(47, 'Jenkins', 7, 48),
(48, 'Postman', 7, 49),
(49, 'Docker', 7, 51)
;

INSERT INTO project_status (id, name)
VALUES (1, 'Idea'),
       (2, 'InProgress'),
       (3, 'Cancelled'),
       (4, 'Completed'),
       (5, 'Pending'),
       (6, 'Archived')
;

INSERT INTO project (id, status_id, title, description, stacks, url_to_project)
VALUES (1, 2, 'portfolio-ssr',
        'Portfolio personnel créé à l''aide du rendu côté serveur (SSR) avec Leptos et Axum, illustrant les capacités modernes de développement web basées sur Rust.',
        (SELECT json_group_array(id)
         FROM technology
         WHERE name IN ('Leptos', 'Rust', 'Axum', 'SQLite', 'Tokio', 'Tracing', 'Refinery', 'Sqlx', 'Docker')),
        'https://github.com/The-Pac/portfolio-ssr'),

       (2, 5, 'R6 Sound Visualizer',
        'Un outil de visualisation sonore spécialement conçu pour Rainbow Six Siege qui fournit un retour visuel sur les signaux audio tout en gérant les niveaux sonores afin d''éviter les dommages auditifs causés par les bruits soudains et forts.',
        (SELECT json_group_array(id) FROM technology WHERE name IN ('Three.Js', 'Node.js')),
        'https://github.com/The-Pac/R6_Sound_Visualization'),

       (3, 5, 'Pac-Gif',
        'Pac-Gif est un projet test visant à créer un outil de capture GIF similaire à Gyazo à l''aide de Tauri, Rust et Svelte. Le projet en est encore à ses débuts et n''est pas encore terminé. Il présente actuellement quelques bugs liés au rendu des couleurs, mais il constitue un bon point de départ pour l''expérimentation et l''apprentissage.',
        (SELECT json_group_array(id) FROM technology WHERE name IN ('Svelte', 'Rust', 'Tauri', 'Vite', 'JavaScript')),
        'https://github.com/The-Pac/pac-gif'),

       (4, 4, 'Superviseur',
        'Superviseur est un projet universitaire développé dans le cadre d''une licence. Le projet porte sur un robot qui suit ses mouvements dans le but de livrer des colis. Le système permet de surveiller la position du robot sur une carte, en fournissant des mises à jour en temps réel.',
        (SELECT json_group_array(id)
         FROM technology
         WHERE name IN ('Java', 'WebSocket', 'JavaScript', 'CSS', 'HTML5', 'Node.js', 'MySQL')),
        'https://github.com/The-Pac/Superviseur'),

       (5, 4, 'Commande SAV Application',
        'Commande SAV Application est un projet développé pendant mon stage dans le cadre de mon programme BTS. L''application a été conçue pour simplifier l''échange de commandes entre les techniciens et la secrétaire.',
        (SELECT json_group_array(id)
         FROM technology
         WHERE name IN ('Java', 'JavaScript', 'Sass', 'CSS', 'HTML5', 'Node.js', 'MySQL')),
        'https://github.com/The-Pac/Commande_SAV_Application'),

       (6, 3, 'Chat box',
        'Chat box est un projet test simple qui établit une connexion au chat Twitch d''un streamer à l''aide du protocole IRC (Internet Relay Chat). Développé en Java avec une interface JavaFX, ce projet montre comment se connecter au système de chat Twitch et effectuer une authentification OAuth de base.',
        (SELECT json_group_array(id) FROM technology WHERE name IN ('Java')),
        'https://github.com/The-Pac/Chat_box'),

       (7, 4, 'Enerdis MEMO4',
        'Il s''agit d''un projet scolaire réalisé par 4 étudiants pendant 4 mois. Le projet complet comprend un site web et une récupération de données via un script PHP via le réseau Things. Je n''ai réalisé que ma partie, qui est une application JavaFX.',
        (SELECT json_group_array(id) FROM technology WHERE name IN ('Java', 'SQLite')),
        'https://github.com/The-Pac/Enerdis_MEMO4')
;

INSERT INTO careers (id, title, year, parent_id, technology_id, logo_id)
VALUES (1, 'Bac', 2019, NULL, NULL, 1),

       (2, 'BTS Systèmes Numériques Informatique et Réseaux', 2019, 1, NULL, 2),
       (3, 'Diplôme BTS', 2021, 2, NULL, 1),
       (4, 'Java', 2020, 2, (SELECT id FROM technology WHERE name = 'Java'), NULL),
       (5, 'Php', 2020, 2, (SELECT id FROM technology WHERE name = 'PHP'), NULL),
       (6, 'Node.Js', 2020, 2, (SELECT id FROM technology WHERE name = 'Node.js'), NULL),
       (7, 'Mysql', 2020, 2, (SELECT id FROM technology WHERE name = 'MySQL'), NULL),
       (8, 'Html', 2020, 2, (SELECT id FROM technology WHERE name = 'HTML5'), NULL),
       (9, 'Css', 2020, 2, (SELECT id FROM technology WHERE name = 'CSS'), NULL),
       (10, 'Windows', 2019, 2, (SELECT id FROM technology WHERE name = 'Windows'), NULL),
       (11, 'Linux', 2019, 2, (SELECT id FROM technology WHERE name = 'Linux'), NULL),
       (12, 'Technopli', 2020, 2, NULL, 31),

       (23, 'Licence Informatique développement', 2021, 3, NULL, 2),
       (24, 'Diplôme Licence Informatique développement', 2022, 23, NULL, 1),
       (13, 'Irouicome', 2022, 23, NULL, 32),
       (14, 'Rest Api', 2022, 23, (SELECT id FROM technology WHERE name = 'REST API'), NULL),
       (15, 'Laravel', 2022, 23, (SELECT id FROM technology WHERE name = 'Laravel'), NULL),
       (16, 'React.Js', 2022, 23, (SELECT id FROM technology WHERE name = 'React'), NULL),
       (17, 'Sqlite', 2022, 23, (SELECT id FROM technology WHERE name = 'SQLite'), NULL),
       (18, 'Tauri', 2022, 23, (SELECT id FROM technology WHERE name = 'Tauri'), NULL),
       (19, 'Rust', 2022, 23, (SELECT id FROM technology WHERE name = 'Rust'), NULL),
       (20, 'Vue.Js', 2022, 23, (SELECT id FROM technology WHERE name = 'Vue.js'), NULL),
       (22, 'Typescript', 2022, 23, (SELECT id FROM technology WHERE name = 'TypeScript'), NULL),
       (41, 'Vite', 2022, 23, (SELECT id FROM technology WHERE name = 'Vite'), NULL),

       (25, 'Astrée software', 2022, 24, NULL, 33),
       (26, 'Spring', 2023, 25, (SELECT id FROM technology WHERE name = 'Spring'), NULL),
       (27, 'Angular', 2023, 25, (SELECT id FROM technology WHERE name = 'Angular'), NULL),
       (29, 'Postgresql', 2023, 25, (SELECT id FROM technology WHERE name = 'PostgreSQL'), NULL),
       (30, 'Svelte', 2023, 25, (SELECT id FROM technology WHERE name = 'Svelte'), NULL),
       (31, 'Axum', 2023, 25, (SELECT id FROM technology WHERE name = 'Axum'), NULL),
       (32, 'Actix', 2023, 25, (SELECT id FROM technology WHERE name = 'Actix'), NULL),

       (33, 'Now', 2024, 25, NULL, 34),
       (28, 'Leptos', 2024, 33, (SELECT id FROM technology WHERE name = 'Leptos'), NULL),
       (42, 'Docker', 2026, 33, (SELECT id FROM technology WHERE name = 'Docker'), NULL)
;

INSERT INTO recommendation (logo_id, author, texte)
VALUES (31, NULL, NULL),
       (32, NULL, NULL),
       (33, NULL, NULL)
;