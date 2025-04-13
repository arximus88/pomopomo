export const WORK_PHRASES = [
    "Вйо працювати!",
    "Час горіти!",
    "До роботи, лінь!",
    "Кодь як бос!",
    "Увага на ціль!",
    "Зараз або ніц!",
    "Ну давай вже!",
    "Не філонь!",
    "Пахати час!",
    "Руки в код!"
];

export const PAUSE_PHRASES = [
    "Дихай, герою!",
    "Кава? Чай?",
    "Розімни кості!",
    "5 хв на себе!",
    "Потягнись!",
    "Відпочинь трохи",
    "Моргни двічі!",
    "Встань з крісла!",
    "Пий водичку!",
    "Не зависай!"
];

export const RELAX_PHRASES = [
    "Ти молодець!",
    "Заслужив!",
    "Вбий дракона!",
    "Вмикай катку!",
    "Відключи мозок",
    "Чіл-аут, братан",
    "Лягай до гамака",
    "Мозок в відпустку",
    "Волю нейронам!",
    "Законний чіл!"
];

export type TimerMode = 'work' | 'break' | 'relax';

export function getRandomPhrase(mode: TimerMode): string {
    const phrases = mode === 'break' ? PAUSE_PHRASES : 
                   mode === 'work' ? WORK_PHRASES : 
                   RELAX_PHRASES;

    const randomIndex = Math.floor(Math.random() * phrases.length);
    return phrases[randomIndex];
} 