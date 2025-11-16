const LoginPage = () => {
    const [email, setEmail] = React.useState('');
    const [password, setPassword] = React.useState('');
    const [error, setError] = React.useState('');

    // URL 쿼리에서 callback 파라미터 추출
    const getCallbackUrl = () => {
        const params = new URLSearchParams(window.location.search);
        return params.get('callback');
    };

    const handleSubmit = async (event) => {
        event.preventDefault();
        setError('');

        try {
            // 백엔드의 로그인 API 호출
            const response = await fetch('/auth/user/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ email, password }),
            });

            if (response.ok) {
                // 로그인 성공 시 callback URL로 리다이렉트
                const callbackUrl = getCallbackUrl();
                if (callbackUrl) {
                    window.location.href = callbackUrl;
                } else {
                    // 콜백이 없으면 메인 페이지로 이동
                    window.location.href = '/';
                }
            } else {
                // 로그인 실패 시 에러 메시지 표시
                const errorData = await response.json();
                setError(errorData.message || '로그인에 실패했습니다.');
            }
        } catch (err) {
            setError('네트워크 오류가 발생했습니다. 잠시 후 다시 시도해주세요.');
        }
    };

    return (
        <div>
            <h1>로그인</h1>
            <form onSubmit={handleSubmit}>
                <input
                    type="text"
                    placeholder="사용자 이름"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    required
                />
                <input
                    type="password"
                    placeholder="비밀번호"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    required
                />
                <button type="submit">로그인</button>
            </form>
            {error && <p className="error">{error}</p>}
        </div>
    );
};

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);
root.render(<LoginPage />);
